use serde::{Deserialize, Serialize};

// https://api.met.no/weatherapi/locationforecast/2.0/#!/data/get_compact_format
// #[derive(Deserialize, Debug, Serialize)]
// struct CompactWeather {}

#[derive(Deserialize, Debug, Serialize)]
pub struct CompleteWeather {
    #[serde(rename = "type")]
    pub r_type: String,
    pub geometry: Geometry,
    pub properties: Properties,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Geometry {
    #[serde(rename = "type")]
    pub r_type: String,
    pub coordinates: Vec<f32>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Properties {
    pub meta: Meta,
    pub timeseries: Vec<WeatherInfo>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Meta {
    pub updated_at: String,
    pub units: Units,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Units {
    pub air_pressure_at_sea_level: String,
    pub air_temperature: String,
    pub air_temperature_max: String,
    pub air_temperature_min: String,
    pub cloud_area_fraction: String,
    pub cloud_area_fraction_high: String,
    pub cloud_area_fraction_low: String,
    pub cloud_area_fraction_medium: String,
    pub dew_point_temperature: String,
    pub fog_area_fraction: String,
    pub precipitation_amount: String,
    pub relative_humidity: String,
    pub ultraviolet_index_clear_sky: String,
    pub wind_from_direction: String,
    pub wind_speed: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct WeatherInfo {
    pub time: String,
    pub data: WeatherData,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct WeatherData {
    pub instant: InstantData,
    pub next_1_hours: Option<NextHoursData>,
    pub next_6_hours: Option<NextHoursData>,
    pub next_12_hours: Option<Next12HoursData>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct InstantData {
    pub details: Details,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Details {
    #[serde(default)]
    pub air_pressure_at_sea_level: f32,
    #[serde(default)]
    pub air_temperature: f32,
    #[serde(default)]
    pub cloud_area_fraction: f32,
    #[serde(default)]
    pub cloud_area_fraction_high: f32,
    #[serde(default)]
    pub cloud_area_fraction_low: f32,
    #[serde(default)]
    pub cloud_area_fraction_medium: f32,
    #[serde(default)]
    pub dew_point_temperature: f32,
    #[serde(default)]
    pub fog_area_fraction: f32,
    #[serde(default)]
    pub relative_humidity: f32,
    #[serde(default)]
    pub ultraviolet_index_clear_sky: f32,
    #[serde(default)]
    pub wind_from_direction: f32,
    #[serde(default)]
    pub wind_speed: f32,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct NextHoursData {
    pub summary: Summary,
    pub details: Data,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Summary {
    pub symbol_code: String,
}


#[derive(Deserialize, Debug, Serialize)]
pub struct Data {
    #[serde(default)]
    pub air_temperature_max: f32,
    #[serde(default)]
    pub air_temperature_min: f32,
    pub precipitation_amount: f32,
}
#[derive(Deserialize, Debug, Serialize)]
pub struct Next12HoursData {
    pub summary: Summary,
}