mod location;

use once_cell::sync::OnceCell;
pub use location::Location as Location;

const IDENT_HEADER: OnceCell<String> = OnceCell::new();

/// Sets the identification header required for <https://yr.no>
/// Read their TOS for allowed headers.
/// Returns `Ok(())` if successful, `Err(header)` if it was alread set.
pub fn set_ident_header(header: String) -> Result<(), String>{
    IDENT_HEADER.set(header)
}

/// Get weather using coordinates.
pub async fn get_weather_at_location(lat: f32, lon: f32) -> Result<(), Box<dyn std::error::Error>>{
    // https://api.met.no/weatherapi/locationforecast/2.0/#!/data/get_complete_format
    let response = reqwest::get(format!("https:://api.met.no/weatherapi/locationforecast/2.0/complete.json?lat={}&lon={}", lat, lon)).await?.text().await?;
    Ok(())
}

#[test]
fn test_header(){
    use dotenv;
    dotenv::dotenv().ok();
    let header = dotenv::var("HEADER").expect("There's no header present in the ENV. Try adding `.env` file and adding variables to it.");
}
