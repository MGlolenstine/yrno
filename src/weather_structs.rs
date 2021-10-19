use serde::{Deserialize, Serialize};

// https://api.met.no/weatherapi/locationforecast/2.0/#!/data/get_compact_format
#[derive(Deserialize, Debug, Serialize)]
pub struct CompactWeather {}

#[derive(Deserialize, Debug, Serialize)]
pub struct CompleteWeather {
    #[serde(rename = "type")]
    r_type: String,
    geometry: Geometry,
    properties: Properties,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Geometry {
    #[serde(rename = "type")]
    r_type: String,
    coordinates: Vec<f32>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Properties {
    meta: Meta,
    timeseries: Vec<WeatherInfo>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Meta {
    updated_at: String,
    units: Units,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Units {
    air_pressure_at_sea_level: String,
    air_temperature: String,
    air_temperature_max: String,
    air_temperature_min: String,
    cloud_area_fraction: String,
    cloud_area_fraction_high: String,
    cloud_area_fraction_low: String,
    cloud_area_fraction_medium: String,
    dew_point_temperature: String,
    fog_area_fraction: String,
    precipitation_amount: String,
    relative_humidity: String,
    ultraviolet_index_clear_sky: String,
    wind_from_direction: String,
    wind_speed: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct WeatherInfo {
    time: String,
    data: WeatherData,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct WeatherData {
    instant: InstantData,
    next_1_hours: Option<NextHoursData>,
    next_6_hours: Option<NextHoursData>,
    next_12_hours: Option<Next12HoursData>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct InstantData {
    details: Details,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Details {
    #[serde(default)]
    air_pressure_at_sea_level: f32,
    #[serde(default)]
    air_temperature: f32,
    #[serde(default)]
    cloud_area_fraction: f32,
    #[serde(default)]
    cloud_area_fraction_high: f32,
    #[serde(default)]
    cloud_area_fraction_low: f32,
    #[serde(default)]
    cloud_area_fraction_medium: f32,
    #[serde(default)]
    dew_point_temperature: f32,
    #[serde(default)]
    fog_area_fraction: f32,
    #[serde(default)]
    relative_humidity: f32,
    #[serde(default)]
    ultraviolet_index_clear_sky: f32,
    #[serde(default)]
    wind_from_direction: f32,
    #[serde(default)]
    wind_speed: f32,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct NextHoursData {
    summary: Summary,
    details: Data,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Summary {
    symbol_code: String,
}


#[derive(Deserialize, Debug, Serialize)]
pub struct Data {
    #[serde(default)]
    air_temperature_max: f32,
    #[serde(default)]
    air_temperature_min: f32,
    precipitation_amount: f32,
}
#[derive(Deserialize, Debug, Serialize)]
pub struct Next12HoursData {
    summary: Summary,
}