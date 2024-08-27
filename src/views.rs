use actix_web::{HttpResponse, web};
use validator::Validate;
use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, Utc};
use serde::Serialize;

use super::{AppState, calculations};
use super::fetch_tle;
use super::forms::{PassesFormData, SatelliteFormData};
use super::serializers::{SerializableGeodedic, SerializableBearing, SerializablePassData};

pub async fn get_passes(data: web::Data<AppState>) -> HttpResponse {
    let mut ctx = tera::Context::new();
    ctx.insert("active_tab", "passes");
    let rendered = data.tera.render("get_passes.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

pub async fn show_passes(form: web::Query<PassesFormData>) -> HttpResponse {
    if let Err(error) = form.validate() {
        return HttpResponse::BadRequest().json(error);
    }

    // let start_time = NaiveDateTime::parse_from_str(&form.start_time, "%Y-%m-%dT%H:%M")
    //     .expect("Parsing can't fail because datetime string was validated")
    //     .and_utc();
    //
    // let passes = match calculations::get_filtered_passes(
    //     &std::env::var("TLE_FILE_PATH").unwrap(),
    //     vec!["METEOR-M2 2", "METEOR-M2 3", "NOAA 18", "NOAA 19", "METOP-B", "METOP-C", "ISS (ZARYA)"],
    //     start_time, Duration::hours(form.duration as i64),
    //     form.min_elevation, form.min_apogee,
    //     form.lat, form.lon, form.alt / 1000.0,
    // ) {
    //     Ok(passes) => passes,
    //     Err(error) => return HttpResponse::BadRequest().body(format!("{}", error)),
    // };

    HttpResponse::Ok().body(format!("{:#?}", "developer gnida"))
}

pub async fn get_satellites_list() -> HttpResponse {
    let tle_fetching_settings = fetch_tle::read_settings().await;

    HttpResponse::Ok().json(tle_fetching_settings.satellites_to_track)
}

pub async fn get_satellite_data(form: web::Query<SatelliteFormData>) -> HttpResponse {
    if let Err(error) = form.validate() {
        return HttpResponse::BadRequest().json(error);
    }

    let start_time = Utc::now();

    let observer = satellite::Geodedic {
        longitude: form.lon * satellite::constants::DEG_2_RAD,
        latitude: form.lat * satellite::constants::DEG_2_RAD,
        height: form.alt,
    };

    macro_rules! unwrap_or_return_response {
        ($result:expr) => {
            match $result {
                Ok(data) => data,
                Err(error) => return HttpResponse::BadRequest().body(format!("{:?}", error))
            }
        };
    }

    let satrec = unwrap_or_return_response!(calculations::find_satrec(
        &std::env::var("TLE_FILE_PATH").unwrap(),
        &form.satellite_name,
    ));

    let norad_id = satrec.satnum.clone();
    let inclination = satrec.inclo * satellite::constants::RAD_TO_DEG;
    let eccentricity = satrec.ecco;
    let period_minutes = satellite::constants::TWO_PI / satrec.no;
    let mean_motion = satrec.no * satellite::constants::RAD_TO_DEG;
    let argument_of_pericenter = satrec.argpo * satellite::constants::RAD_TO_DEG;
    let mean_anomaly = satrec.mo * satellite::constants::RAD_TO_DEG;
    let raan = satrec.nodeo * satellite::constants::RAD_TO_DEG;

    let year = if satrec.epochyr < 57 { 2000 + satrec.epochyr } else { 1900 + satrec.epochyr };
    let start_of_year_date = NaiveDate::from_ymd_opt(year as i32, 1, 1)
        .expect("January 1st of year Satrec.epochyr should exist");
    let start_of_year = NaiveDateTime::from(start_of_year_date);
    let epoch_naive = start_of_year + Duration::seconds((satrec.epochdays * 86400.0) as i64);
    let epoch = epoch_naive.and_utc();

    let is_geostationary = (period_minutes - 1436.0).abs() < 10.0;

    let trajectory = unwrap_or_return_response!(calculations::get_trajectory(
        &satrec, start_time - Duration::hours(1), Duration::hours(2),
    )).into_iter().map(Into::into).collect();

    let look_angles = unwrap_or_return_response!(calculations::get_observer_trajectory(
        &satrec, start_time, Duration::hours(1), &observer,
    )).into_iter().map(Into::into).collect();

    let passes;

    if !is_geostationary {
        passes = unwrap_or_return_response!(calculations::get_satellite_passes(
            &satrec, start_time, Duration::hours(24), &observer,
        )).into_iter().map(Into::into).collect();
    } else {
        passes = vec![];
    }

    #[derive(Serialize)]
    struct SatelliteData {
        norad_id: String,
        inclination: f64,
        eccentricity: f64,
        period_minutes: f64,
        mean_motion: f64,
        argument_of_pericenter: f64,
        mean_anomaly: f64,
        raan: f64,  // Долгота восходящего угла
        epoch: DateTime<Utc>,
        is_geostationary: bool,
        trajectory: Vec<SerializableGeodedic>,
        look_angles: Vec<SerializableBearing>,
        passes: Vec<SerializablePassData>,
    }

    let response = SatelliteData {
        norad_id,
        inclination,
        eccentricity,
        period_minutes,
        mean_motion,
        argument_of_pericenter,
        mean_anomaly,
        raan,
        epoch,
        is_geostationary,
        trajectory,
        look_angles,
        passes,
    };

    HttpResponse::Ok().json(response)
}
