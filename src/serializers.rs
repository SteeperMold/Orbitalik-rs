use chrono::{DateTime, Utc};
use serde::Serialize;
use satellite::{Geodedic, Bearing};
use super::calculations::PassData;

#[derive(Serialize)]
pub struct SerializablePassData {
    pub satellite_name: String,
    pub rise_time: DateTime<Utc>,
    pub rise_azimuth: f64,
    pub fall_time: DateTime<Utc>,
    pub fall_azimuth: f64,
    pub apogee_time: DateTime<Utc>,
    pub apogee_elevation: f64,
    pub apogee_azimuth: f64,
}

impl From<PassData> for SerializablePassData {
    fn from(pass_data: PassData) -> Self {
        SerializablePassData {
            satellite_name: pass_data.satellite_name,
            rise_time: pass_data.rise_time,
            rise_azimuth: pass_data.rise_azimuth,
            fall_time: pass_data.fall_time,
            fall_azimuth: pass_data.fall_azimuth,
            apogee_time: pass_data.apogee_time,
            apogee_elevation: pass_data.apogee_elevation,
            apogee_azimuth: pass_data.apogee_azimuth,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct SerializableGeodedic {
    pub lon: f64,
    pub lat: f64,
    pub alt: f64,
}

impl From<Geodedic> for SerializableGeodedic {
    fn from(geodedic: Geodedic) -> Self {
        SerializableGeodedic {
            lon: geodedic.longitude,
            lat: geodedic.latitude,
            alt: geodedic.height,
        }
    }
}

#[derive(Serialize)]
pub struct SerializableBearing {
    pub az: f64,
    pub el: f64,
}

impl From<Bearing> for SerializableBearing {
    fn from(bearing: Bearing) -> Self {
        SerializableBearing {
            az: bearing.azimuth,
            el: bearing.elevation,
        }
    }
}
