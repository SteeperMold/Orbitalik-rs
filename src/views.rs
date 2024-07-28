use actix_web::{HttpResponse, web};
use validator::Validate;
use chrono::{Duration, NaiveDateTime};

use super::AppState;
use super::forms::FormData;
use super::calculations::{get_satellite_passes, get_observer_look, get_filtered_passes};

pub async fn get_passes(data: web::Data<AppState>) -> HttpResponse {
    let mut ctx = tera::Context::new();
    ctx.insert("active_tab", "passes");
    let rendered = data.tmpl.render("get_passes.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

pub async fn show_passes(form: web::Query<FormData>) -> HttpResponse {
    if let Err(error) = form.validate() {
        return HttpResponse::BadRequest().json(error);
    }

    let start_time = NaiveDateTime::parse_from_str(&form.start_time, "%Y-%m-%dT%H:%M")
        .expect("failed to parse datetime")
        .and_utc();

    let passes = match get_filtered_passes(
        "data/tle.txt", vec!["ISS (ZARYA)", "NOAA 19", "METEOR-M 2"],
        start_time, Duration::hours(form.duration as i64),
        form.min_elevation, form.min_apogee,
        form.lat, form.lon, form.alt / 1000.0,
    ) {
        Ok(passes) => passes,
        Err(error) => return HttpResponse::BadRequest().body(format!("{}", error))
    };

    HttpResponse::Ok().body(format!("{:#?}", passes))
}
