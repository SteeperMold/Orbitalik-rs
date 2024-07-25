use actix_web::{error, Error, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};
use chrono::naive::NaiveDateTime;

pub fn parse_error_handler(err: error::UrlencodedError, _req: &HttpRequest) -> Error {
    error::InternalError::from_response(
        format!("ParseError handler was called with error {}", err),
        HttpResponse::BadRequest()
            .content_type("application/json")
            .body(format!(r#"{{"error": "{}"}}"#, err)),
    ).into()
}

fn validate_local_datetime(value: &str) -> Result<(), ValidationError> {
    match NaiveDateTime::parse_from_str(value, "%Y-%m-%dT%H:%M") {
        Ok(_) => Ok(()),
        Err(_) => Err(ValidationError::new("Datetime validation failed"))
    }
}

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct FormData {
    #[validate(range(min = 0.0, max = 360.0, message = "Широта должна быть от 0 до 360"))]
    pub lat: f64,
    #[validate(range(min = 0.0, max = 90.0, message = "Долгота должна быть от 0 до 90"))]
    pub lon: f64,
    #[validate(range(min = 0.0, max = 10000.0, message = "Слишком большая высота"))]
    pub alt: f64,
    #[validate(range(min = 0.0, max = 90.0, message = "Элевация должна быть от 0 до 90"))]
    pub min_elevation: f64,
    #[validate(range(min = 0.0, max = 90.0, message = "Кульминация должна быть от 0 до 90"))]
    pub min_apogee: f64,
    #[validate(custom(function = "validate_local_datetime", message = "Неверный формат времени"))]
    pub start_time: String,
    #[validate(range(min = 1, max = 240, message = "Длительность наблюдения не должна превышать 10 дней"))]
    pub duration: u8,
}