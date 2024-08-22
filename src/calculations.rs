use chrono::{DateTime, Duration, Utc};
use roots::{find_root_brent, SimpleConvergency};
use thiserror::Error;

#[derive(Debug, Error)]
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
    pub satellite_name: String,
    pub rise_time: DateTime<Utc>,
    pub rise_azimuth: f64,
    pub apogee_time: DateTime<Utc>,
    pub apogee_elevation: f64,
    pub apogee_azimuth: f64,
    pub fall_time: DateTime<Utc>,
    pub fall_azimuth: f64,
}

pub fn find_satrec(
    tle_file_path: &str, satellite_name: &str,
) -> Result<satellite::io::Satrec, PassesCalculationError> {
    let tle = std::fs::read_to_string(tle_file_path)?;
    let (satrecs, _errors) = satellite::io::parse_multiple(&tle);

    for possible_satrec in satrecs {
        if let Some(possible_name) = &possible_satrec.name {
            if possible_name == satellite_name {
                return Ok(possible_satrec);
            }
        }
    }

    Err(PassesCalculationError::SatelliteNotFound)
}

fn get_elevation_safe(
    satrec: &satellite::io::Satrec,
    observer: &satellite::Geodedic,
    time: DateTime<Utc>,
) -> f64 {
    let propogation = match satellite::propogation::propogate_datetime(satrec, time) {
        Ok(propogation) => propogation,
        Err(_) => return f64::NAN
    };
    let gmst = satellite::propogation::gstime::gstime_datetime(time);
    let position_ecf = satellite::transforms::eci_to_ecf(&propogation.position, gmst);
    let look_angles = satellite::transforms::ecf_to_look_angles(observer, &position_ecf);
    look_angles.elevation
}

fn get_observer_look(
    satrec: &satellite::io::Satrec,
    time: DateTime<Utc>,
    observer: &satellite::Geodedic,
) -> Result<satellite::Bearing, PassesCalculationError> {
    let propogation = match satellite::propogation::propogate_datetime(satrec, time) {
        Ok(propogation) => propogation,
        Err(_) => return Err(PassesCalculationError::PropogationError),
    };

    let gmst = satellite::propogation::gstime::gstime_datetime(time);
    let position_ecf = satellite::transforms::eci_to_ecf(&propogation.position, gmst);
    let mut look_angles = satellite::transforms::ecf_to_look_angles(&observer, &position_ecf);

    look_angles.elevation *= satellite::constants::RAD_TO_DEG;
    look_angles.azimuth *= satellite::constants::RAD_TO_DEG;

    Ok(look_angles)
}

pub fn get_satellite_pos(
    satrec: &satellite::io::Satrec,
    time: DateTime<Utc>,
) -> Result<satellite::Geodedic, PassesCalculationError> {
    let propogation = match satellite::propogation::propogate_datetime(satrec, time) {
        Ok(propogation) => propogation,
        Err(_) => return Err(PassesCalculationError::PropogationError),
    };

    let gmst = satellite::propogation::gstime::gstime_datetime(time);
    let mut sat_pos = satellite::transforms::eci_to_geodedic(&propogation.position, gmst);

    sat_pos.longitude *= satellite::constants::RAD_TO_DEG;
    sat_pos.latitude *= satellite::constants::RAD_TO_DEG;

    Ok(sat_pos)
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
    satrec: &satellite::io::Satrec,
    start_time: DateTime<Utc>, duration: Duration,
    lat: f64, lon: f64, alt: f64,
) -> Result<Vec<PassData>, PassesCalculationError> {
    let observer = satellite::Geodedic {
        longitude: lon * satellite::constants::DEG_2_RAD,
        latitude: lat * satellite::constants::DEG_2_RAD,
        height: alt,
    };

    let mut get_elevation = |shift_minutes: f64| -> f64 {
        let current_time = start_time + Duration::seconds((shift_minutes * 60.0) as i64);
        get_elevation_safe(&satrec, &observer, current_time)
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

                    let rise_azimuth = get_observer_look(&satrec, rt, &observer)?.azimuth;
                    let fall_azimuth = get_observer_look(&satrec, fall_time, &observer)?.azimuth;
                    let apogee_azimuth = get_observer_look(&satrec, apogee_time, &observer)?.azimuth;

                    let pass = PassData {
                        satellite_name: satrec.name.clone().unwrap_or("N/A".to_string()),
                        rise_time: rt,
                        rise_azimuth,
                        apogee_time,
                        apogee_elevation,
                        apogee_azimuth,
                        fall_time,
                        fall_azimuth,
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

pub fn get_filtered_passes(
    satrecs: Vec<&satellite::io::Satrec>,
    start_time: DateTime<Utc>, duration: Duration,
    min_elevation: f64, min_apogee: f64,
    lat: f64, lon: f64, alt: f64,
) -> Result<Vec<PassData>, PassesCalculationError> {
    let min_elevation_rad = min_elevation * satellite::constants::DEG_2_RAD;

    let observer = satellite::Geodedic {
        longitude: lon * satellite::constants::DEG_2_RAD,
        latitude: lat * satellite::constants::DEG_2_RAD,
        height: alt,
    };

    let mut all_passes = vec![];

    for satrec in satrecs {
        let passes = get_satellite_passes(
            satrec,
            start_time, duration,
            lat, lon, alt,
        )?;

        let mut passes: Vec<PassData> = passes.into_iter()
            .filter(|pass_data| {
                pass_data.apogee_elevation >= min_apogee &&
                    pass_data.apogee_elevation >= min_elevation
            })
            .collect();

        for pass_data in &mut passes {
            let rise_time = pass_data.rise_time;

            let get_elevation = |shift_minutes: f64| -> f64 {
                let current_time = rise_time + Duration::seconds((shift_minutes * 60.0) as i64);
                get_elevation_safe(satrec, &observer, current_time)
            };

            let mut prev_elevation = get_elevation(0.0);
            let pass_duration = (pass_data.fall_time - pass_data.rise_time).num_seconds() as f64 / 60.0;

            for shift in 1..=pass_duration as i64 {
                let current_elevation = get_elevation(shift as f64);

                if prev_elevation <= min_elevation_rad && min_elevation_rad <= current_elevation {
                    let rise_mins = get_root(
                        |shift| { get_elevation(shift) - min_elevation_rad },
                        (shift - 1) as f64,
                        shift as f64,
                    )?;

                    pass_data.rise_time += Duration::seconds((rise_mins * 60.0) as i64);
                    break;
                }

                prev_elevation = current_elevation;
            }

            prev_elevation = get_elevation(pass_duration);

            for shift in (0..=pass_duration as i64 - 1).rev() {
                let current_elevation = get_elevation(shift as f64) - min_elevation_rad;

                if prev_elevation.is_sign_negative() && current_elevation.is_sign_positive() {
                    let fall_mins = get_root(
                        |shift| { get_elevation(shift) - min_elevation_rad },
                        (shift + 2) as f64,  // +2 на случай если длительность пролета дробная и
                        shift as f64,        // момент смены знака приходиться на дробную чатсть
                    )?;

                    let fall_mins_from_end = pass_duration - fall_mins;
                    pass_data.fall_time -= Duration::seconds((fall_mins_from_end * 60.0) as i64);
                    break;
                }

                prev_elevation = current_elevation;
            }
        }

        all_passes.extend(passes);
    }

    all_passes.sort_by_key(|pass_data| pass_data.rise_time);

    Ok(all_passes)
}

pub fn get_trajectory(
    satrec: &satellite::io::Satrec,
    start_time: DateTime<Utc>, duration: Duration,
) -> Result<Vec<satellite::Geodedic>, PassesCalculationError> {
    let mut result = vec![];

    for shift in 0..duration.num_seconds() {
        let current_time = start_time + Duration::seconds(shift);

        let sat_pos = get_satellite_pos(&satrec, current_time)?;

        result.push(sat_pos);
    }

    Ok(result)
}

pub fn get_observer_trajectory(
    satrec: &satellite::io::Satrec,
    start_time: DateTime<Utc>, duration: Duration,
    lat: f64, lon: f64, alt: f64,
) -> Result<Vec<satellite::Bearing>, PassesCalculationError> {
    let observer = satellite::Geodedic {
        longitude: lon * satellite::constants::DEG_2_RAD,
        latitude: lat * satellite::constants::DEG_2_RAD,
        height: alt,
    };

    let mut result = vec![];

    for shift in 1..=duration.num_seconds() {
        let current_time = start_time + Duration::seconds(shift);

        let look_angles = get_observer_look(&satrec, current_time, &observer)?;

        result.push(look_angles);
    }

    Ok(result)
}

