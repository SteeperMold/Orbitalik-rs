use actix_web::{HttpResponse, web};
use validator::Validate;
use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, Utc};
use serde::Serialize;

use super::{calculations, fetch_tle};
use super::forms::{PassesListForm, SatelliteDataForm, TrajectoryForm};
use super::serializers::{SerializableGeodedic, SerializableBearing, SerializablePassData};

macro_rules! unwrap_or_return_response {
    ($result:expr) => {
        match $result {
            Ok(data) => data,
            Err(_) => return HttpResponse::BadRequest()
                .body("{\"error\": \"Failed to calculate satellite data\"}")
        }
    };
}

pub async fn get_satellites_list() -> HttpResponse {
    let tle_fetching_settings = fetch_tle::read_settings().await;

    HttpResponse::Ok().json(tle_fetching_settings.satellites_to_track)
}

pub async fn get_satellite_data(form: web::Query<SatelliteDataForm>) -> HttpResponse {
    if let Err(error) = form.validate() {
        return HttpResponse::BadRequest().json(error);
    }

    let start_time = Utc::now();

    let observer = satellite::Geodedic {
        longitude: form.lon * satellite::constants::DEG_2_RAD,
        latitude: form.lat * satellite::constants::DEG_2_RAD,
        height: form.alt / 1000.0,
    };

    let satrec = unwrap_or_return_response!(calculations::find_satrec(
        &std::env::var("TLE_FILE_PATH").unwrap(),
        &form.satellite_name,
    ));

    let satellite_name = satrec.name.clone().unwrap_or("Unknown satellite".to_string());
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
        satellite_name: String,
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
        satellite_name,
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

pub async fn get_passes_list(form: web::Query<PassesListForm>) -> HttpResponse {
    if let Err(error) = form.validate() {
        return HttpResponse::BadRequest().json(error);
    }

    let tle_file_path = &std::env::var("TLE_FILE_PATH").unwrap();

    let satrecs: Vec<_> = form.satellites.split(",")
        .map(|satellite| {
            calculations::find_satrec(&tle_file_path, satellite)
                .expect("Satrec should be found because satellites list was validated")
        })
        .collect();

    let start_time = NaiveDateTime::parse_from_str(&form.start_time, "%Y-%m-%dT%H:%M")
        .expect("Parsing shouldn't fail because datetime string was validated")
        .and_utc();

    let duration = Duration::hours(form.duration as i64);

    let observer = satellite::Geodedic {
        latitude: form.lat * satellite::constants::DEG_2_RAD,
        longitude: form.lon * satellite::constants::DEG_2_RAD,
        height: form.alt / 1000.0,
    };

    let passes = unwrap_or_return_response!(calculations::get_filtered_passes(
        satrecs,
        start_time, duration,
        form.min_elevation, form.min_apogee,
        &observer,
    ));

    HttpResponse::Ok().json(passes)
}

pub async fn get_trajectory(form: web::Query<TrajectoryForm>) -> HttpResponse {
    if let Err(error) = form.validate() {
        return HttpResponse::BadRequest().json(error);
    }

    let satrec = unwrap_or_return_response!(calculations::find_satrec(
        &std::env::var("TLE_FILE_PATH").unwrap(),
        &form.satellite,
    ));

    let start_time = NaiveDateTime::parse_from_str(&form.start_time, "%Y-%m-%dT%H:%M")
        .expect("Parsing shouldn't fail because datetime string was validated")
        .and_utc();

    let end_time = NaiveDateTime::parse_from_str(&form.end_time, "%Y-%m-%dT%H:%M")
        .expect("Parsing shouldn't fail because datetime string was validated")
        .and_utc();

    let duration = end_time - start_time;

    let observer = satellite::Geodedic {
        latitude: form.lat * satellite::constants::DEG_2_RAD,
        longitude: form.lon * satellite::constants::DEG_2_RAD,
        height: form.alt / 1000.0,
    };

    let trajectory = unwrap_or_return_response!(calculations::get_trajectory(
        &satrec, start_time, duration,
    )).into_iter().map(Into::into).collect();

    let look_angles = unwrap_or_return_response!(calculations::get_observer_trajectory(
        &satrec, start_time, duration, &observer,
    )).into_iter().map(Into::into).collect();

    #[derive(Serialize)]
    struct TrajectoryData {
        trajectory: Vec<SerializableGeodedic>,
        look_angles: Vec<SerializableBearing>,
    }

    let response = TrajectoryData {
        trajectory,
        look_angles,
    };

    HttpResponse::Ok().json(response)
}

