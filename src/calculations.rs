use chrono::{DateTime, Duration, Utc};
use roots::{find_root_brent, SimpleConvergency};

#[derive(Debug, thiserror::Error)]
pub enum PassesCalculationError {
    #[error("Failed to open tle at the specified path")]
    TleLoadingFailed(#[from] std::io::Error),
    #[error("Failed to find the specified satellite at the specified tle path")]
    SatelliteNotFound,
    #[error("Failed to find function root")]
    RootCalculationError(#[from] roots::SearchError),
    #[error("Failed to propogate satellite")]
    PropogationError,
}

#[derive(Debug)]
pub struct PassData {
    pub rise_time: DateTime<Utc>,
    pub fall_time: DateTime<Utc>,
    pub apogee_time: DateTime<Utc>,
    pub apogee_elevation: f64,
}

fn find_satrec(tle_file_path: &str, satellite_name: &str)
               -> Result<satellite::io::Satrec, PassesCalculationError> {
    let tle = std::fs::read_to_string(tle_file_path)?;
    let (satrecs, _errors) = satellite::io::parse_multiple(&tle);

    let mut satrec = None;
    for possible_satrec in satrecs {
        if let Some(possible_name) = &possible_satrec.name {
            if possible_name == satellite_name {
                satrec = Some(possible_satrec);
                break;
            }
        }
    }

    match satrec {
        Some(satrec) => Ok(satrec),
        None => Err(PassesCalculationError::SatelliteNotFound)
    }
}

fn get_max_parab<F>(mut fun: F, start: f64, end: f64, tol: f64) -> f64
    where
        F: FnMut(f64) -> f64
{
    let mut a = start;
    let mut c = end;
    let mut b = (a + c) / 2.0;

    let mut f_a = fun(a);
    let mut f_b = fun(b);
    let mut f_c = fun(c);

    let x = b;

    loop {
        let numerator = (b - a).powi(2) * (f_b - f_c) - (b - c).powi(2) * (f_b - f_a);
        let denominator = (b - a) * (f_b - f_c) - (b - c) * (f_b - f_a);

        if denominator == 0.0 {
            return b;
        }

        let x = x - 0.5 * (numerator / denominator);

        if (b - x).abs() <= tol {
            return x;
        }

        let f_x = fun(x);

        if f_x > f_b {
            return b;
        }

        (a, b, c) = ((a + x) / 2.0, x, (x + c) / 2.0);
        (f_a, f_b, f_c) = (fun(a), f_x, fun(c));
    }
}

fn get_root<F>(mut fun: F, start: f64, end: f64) -> Result<f64, PassesCalculationError>
    where
        F: FnMut(f64) -> f64
{
    let mut x_0 = end;
    let mut x_1 = start;

    if fun(x_0).abs() < fun(x_1).abs() {
        std::mem::swap(&mut x_0, &mut x_1);
    }

    let mut convergency = SimpleConvergency { eps: 2e-12, max_iter: 200 };

    match find_root_brent(x_0, x_1, &mut fun, &mut convergency) {
        Ok(root) => Ok(root),
        Err(error) => Err(PassesCalculationError::RootCalculationError(error))
    }
}

pub fn get_satellite_passes(
    tle_file_path: &str, satellite_name: &str,
    start_time: DateTime<Utc>, duration: Duration,
    lat: f64, lon: f64, alt: f64,
) -> Result<Vec<PassData>, PassesCalculationError> {
    let mut satrec = find_satrec(tle_file_path, satellite_name)?;

    let observer = satellite::Geodedic {
        longitude: lon * satellite::constants::DEG_2_RAD,
        latitude: lat * satellite::constants::DEG_2_RAD,
        height: alt,
    };

    let mut get_elevation = |shift_minutes: f64| -> f64 {
        let current_time = start_time + Duration::seconds((shift_minutes * 60.0) as i64);
        let propogation = match satellite::propogation::propogate_datetime(&mut satrec, current_time) {
            Ok(propogation) => propogation,
            Err(_) => return f64::NAN
        };
        let gmst = satellite::propogation::gstime::gstime_datetime(current_time);
        let position_ecf = satellite::transforms::eci_to_ecf(&propogation.position, gmst);
        let look_angles = satellite::transforms::ecf_to_look_angles(&observer, &position_ecf);
        look_angles.elevation
    };

    let mut result = vec![];

    let mut rise_mins = 0.0;
    let mut rise_time = None;

    let mut prev_elevation = get_elevation(0.0);

    for shift in 1..=duration.num_minutes() {
        let curr_elevation = get_elevation(shift as f64);

        if curr_elevation.is_sign_positive() != prev_elevation.is_sign_positive() {
            let horizon_mins = get_root(&mut get_elevation, (shift - 1) as f64, shift as f64)?;
            let horizon_time = start_time + Duration::seconds((horizon_mins * 60.0) as i64);

            if prev_elevation.is_sign_negative() {
                rise_mins = horizon_mins;
                rise_time = Some(horizon_time);
            } else {
                let fall_mins = horizon_mins;
                let fall_time = horizon_time;

                if let Some(rt) = rise_time {
                    let mut max_elev = 0.0;
                    let mut middle_mins = 0;

                    for i in rise_mins as i64..=fall_mins as i64 {
                        let curr_elev = get_elevation(i as f64);

                        if curr_elev > max_elev {
                            max_elev = curr_elev;
                            middle_mins = i;
                        }
                    }

                    let apogee_mins = get_max_parab(
                        |x| { -get_elevation(x) },
                        f64::max(rise_mins, (middle_mins - 1) as f64),
                        f64::min(fall_mins, (middle_mins + 1) as f64),
                        0.001 / 60.0,
                    );

                    let apogee_elevation = get_elevation(apogee_mins) * satellite::constants::RAD_TO_DEG;
                    let apogee_time = start_time + Duration::seconds((60.0 * apogee_mins) as i64);

                    let pass = PassData {
                        rise_time: rt,
                        fall_time,
                        apogee_time,
                        apogee_elevation,
                    };

                    result.push(pass);
                    rise_time = None;
                }
            }
        }

        prev_elevation = curr_elevation;
    }

    Ok(result)
}

pub fn get_observer_look(
    tle_file_path: &str, satellite_name: &str,
    time: DateTime<Utc>,
    lat: f64, lon: f64, alt: f64,
) -> Result<satellite::Bearing, PassesCalculationError> {
    let mut satrec = find_satrec(tle_file_path, satellite_name)?;

    let observer = satellite::Geodedic {
        longitude: lon * satellite::constants::DEG_2_RAD,
        latitude: lat * satellite::constants::DEG_2_RAD,
        height: alt,
    };

    let propogation = match satellite::propogation::propogate_datetime(&mut satrec, time) {
        Ok(propogation) => propogation,
        Err(_) => return Err(PassesCalculationError::PropogationError)
    };
    let gmst = satellite::propogation::gstime::gstime_datetime(time);
    let position_ecf = satellite::transforms::eci_to_ecf(&propogation.position, gmst);
    let look_angles = satellite::transforms::ecf_to_look_angles(&observer, &position_ecf);

    Ok(look_angles)
}

pub fn get_filtered_passes(
    tle_file_path: &str, satellite_names: Vec<&str>,
    start_time: DateTime<Utc>, duration: Duration,
    min_elevation: f64, min_apogee: f64,
    lat: f64, lon: f64, alt: f64,
) -> Result<Vec<PassData>, PassesCalculationError> {
    let mut all_passes = vec![];

    for satellite_name in satellite_names {
        let passes = get_satellite_passes(
            tle_file_path, satellite_name,
            start_time, duration,
            lat, lon, alt,
        )?;

        let passes: Vec<PassData> = passes.into_iter()
            .filter(|pass_data| pass_data.apogee_elevation >= min_apogee)
            .collect();

        let passes: Vec<PassData> = passes.into_iter()
            .map(|| {})
            .collect();

    }

    Ok(all_passes)
}


