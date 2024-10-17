use actix_web::{error, Error, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};
use chrono::naive::NaiveDateTime;

use super::fetch_tle;

pub fn parse_error_handler(err: error::UrlencodedError, _req: &HttpRequest) -> Error {
    error::InternalError::from_response(
        format!("ParseError handler was called with error {}", err),
        HttpResponse::BadRequest()
            .content_type("application/json")
            .body(format!(r#"{{"error": "{}"}}"#, err)),
    ).into()
}

fn validate_datetime(value: &str) -> Result<(), ValidationError> {
    match NaiveDateTime::parse_from_str(value, "%Y-%m-%dT%H:%M") {
        Ok(_) => Ok(()),
        Err(_) => Err(ValidationError::new("Datetime validation failed"))
    }
}

fn validate_satellites_list(satellites: &str) -> Result<(), ValidationError> {
    let valid_satellites = fetch_tle::read_settings_sync().satellites_to_track;

    if satellites.split(",").all(|item| valid_satellites.contains(&item.to_string())) {
        Ok(())
    } else {
        Err(ValidationError::new("Satellites list validation failed"))
    }
}

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct SatelliteDataForm {
    #[validate(length(min = 1, max = 32))]
    pub satellite_name: String,
    #[validate(range(min = - 180.0, max = 180.0, message = "Широта должна быть от -180 до 180"))]
    pub lat: f64,
    #[validate(range(min = - 90.0, max = 90.0, message = "Долгота должна быть от -90 до 90"))]
    pub lon: f64,
    #[validate(range(min = 0.0, max = 10000.0, message = "Слишком большая высота"))]
    pub alt: f64,
}

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct PassesListForm {
    #[validate(custom(function = "validate_satellites_list"))]
    pub satellites: String,
    #[validate(range(min = - 180.0, max = 180.0, message = "Широта должна быть от -180 до 180"))]
    pub lat: f64,
    #[validate(range(min = - 90.0, max = 90.0, message = "Долгота должна быть от -90 до 90"))]
    pub lon: f64,
    #[validate(range(min = 0.0, max = 10000.0, message = "Слишком большая высота"))]
    pub alt: f64,
    #[validate(range(min = 0.0, max = 90.0, message = "Элевация должна быть от 0 до 90"))]
    pub min_elevation: f64,
    #[validate(range(min = 0.0, max = 90.0, message = "Кульминация должна быть от 0 до 90"))]
    pub min_apogee: f64,
    #[validate(custom(function = "validate_datetime", message = "Неверный формат времени"))]
    pub start_time: String,
    #[validate(range(min = 1, max = 240, message = "Длительность наблюдения не должна превышать 10 дней"))]
    pub duration: u8,
}


// TODO: сделать так чтобы старт тайм и энд тайм были оциаональными параметрами и если их нет, то по дефолту как в форме сверху
#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct TrajectoryForm {
    #[validate(custom(function = "validate_satellites_list"))]
    pub satellite: String,
    #[validate(range(min = - 180.0, max = 180.0, message = "Широта должна быть от -180 до 180"))]
    pub lat: f64,
    #[validate(range(min = - 90.0, max = 90.0, message = "Долгота должна быть от -90 до 90"))]
    pub lon: f64,
    #[validate(range(min = 0.0, max = 10000.0, message = "Слишком большая высота"))]
    pub alt: f64,
    #[validate(custom(function = "validate_datetime", message = "Неверный формат времени"))]
    pub start_time: String,
    #[validate(custom(function = "validate_datetime", message = "Неверный формат времени"))]
    pub end_time: String,
}
