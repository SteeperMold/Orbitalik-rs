use actix_web::{HttpResponse, web};
use validator::Validate;
use chrono::{Duration, Utc};
use serde::Serialize;

use super::{AppState, calculations};
use super::forms::{PassesFormData, SatelliteFormData};
use super::fetch_tle::{read_settings};
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
    let path = std::env::var("TLE_FETCHING_SETTINGS_PATH").unwrap();
    let tle_fetching_settings = read_settings(&path);

    HttpResponse::Ok().json(tle_fetching_settings.satellites_to_track)
}

pub async fn get_satellite_data(form: web::Query<SatelliteFormData>) -> HttpResponse {
    if let Err(error) = form.validate() {
        return HttpResponse::BadRequest().json(error);
    }

    let tle_path = &std::env::var("TLE_FILE_PATH").unwrap();
    let satrec = match calculations::find_satrec(tle_path, &form.satellite_name) {
        Ok(satrec) => satrec,
        Err(error) => return HttpResponse::BadRequest().body(format!("{:?}", error)),
    };
    let start_time = Utc::now();

    let trajectory = match calculations::get_trajectory(
        &satrec, start_time - Duration::hours(1), Duration::hours(2),
    ) {
        Ok(trajectory) => trajectory,
        Err(error) => return HttpResponse::BadRequest().body(format!("{:?}", error)),
    }.into_iter().map(Into::into).collect();

    let look_angles = match calculations::get_observer_trajectory(
        &satrec,
        start_time, Duration::hours(1),
        form.lat, form.lon, form.alt,
    ) {
        Ok(passes) => passes,
        Err(error) => return HttpResponse::BadRequest().body(format!("{:?}", error)),
    }.into_iter().map(Into::into).collect();

    let passes = match calculations::get_satellite_passes(
        &satrec,
        start_time, Duration::hours(24),
        form.lat, form.lon, form.alt,
    ) {
        Ok(passes) => passes,
        Err(error) => return HttpResponse::BadRequest().body(format!("{:?}", error)),
    }.into_iter().map(Into::into).collect();

    let norad_id: u64 = satrec.satnum.parse().expect("Satrec.satnum should always be a number");

    #[derive(Serialize)]
    struct SatelliteData {
        norad_id: u64,
        trajectory: Vec<SerializableGeodedic>,
        look_angles: Vec<SerializableBearing>,
        passes: Vec<SerializablePassData>,
    }

    let response = SatelliteData {
        norad_id,
        trajectory,
        look_angles,
        passes,
    };

    HttpResponse::Ok().json(response)
}
