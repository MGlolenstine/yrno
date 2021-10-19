pub mod error;
mod location;
mod weather_structs;
use error::WeatherError;
pub use location::Location;
pub use weather_structs::*;

use once_cell::sync::OnceCell;
use reqwest::{Client, RequestBuilder};

static IDENT_HEADER: OnceCell<String> = OnceCell::new();
static YRNO_CLIENT: OnceCell<Client> = OnceCell::new();

/// Sets the identification header required for <https://yr.no>
/// Read their TOS for allowed headers.
/// Returns `Ok(())` if successful, `Err(WeatherError::HeaderAlreadySet)` if it was alread set.
pub fn set_ident_header(header: String) -> Result<(), Box<dyn std::error::Error>> {
    if IDENT_HEADER.set(header).is_err(){
        Err(Box::new(WeatherError::HeaderAlreadySet))
    }else{
        Ok(())
    }
}

/// Get weather using coordinates.
pub async fn get_weather_at_location(
    lat: f32,
    lon: f32,
) -> Result<CompleteWeather, Box<dyn std::error::Error>> {
    // https://api.met.no/weatherapi/locationforecast/2.0/#!/data/get_complete_format
    let url = format!(
        "https://api.met.no/weatherapi/locationforecast/2.0/complete.json?lat={:.4}&lon={:.4}",
        lat, lon
    );
    match get_weather_query(&url) {
        Ok(request) => {
            let response: CompleteWeather = request.send().await?.json().await?;
            Ok(response)
        }
        Err(e) => Err(e),
    }
}

// /// Get compact weather using coordinates.
// pub async fn get_compact_weather_at_location(
//     lat: f32,
//     lon: f32,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     // https://api.met.no/weatherapi/locationforecast/2.0/#!/data/get_compact_format
//     let url = format!(
//         "https://api.met.no/weatherapi/locationforecast/2.0/compact.json?lat={:.4}&lon={:.4}",
//         lat, lon
//     );
//     if let Ok(request) = get_weather_query(&url) {
//         let response = request.send().await?.text().await?;
//         Ok(())
//     } else {
//         Ok(())
//     }
// }

fn get_weather_query(url: &str) -> Result<RequestBuilder, Box<dyn std::error::Error>> {
    // Get the client if it doesn't exist yet.
    let client = YRNO_CLIENT.get_or_init(Client::new);
    if let Some(header) = IDENT_HEADER.get() {
        let request = client.get(url).header("User-Agent", header);
        Ok(request)
    } else {
        Err(Box::new(WeatherError::RequestHeaderNotSet))
    }
}

#[tokio::test]
async fn test_header() {
    use dotenv;
    dotenv::dotenv().ok();
    let header = dotenv::var("HEADER").expect(
        "There's no header present in the ENV. Try adding `.env` file and adding variables to it.",
    );
    assert!(set_ident_header(header.clone()).is_ok());
    assert!(set_ident_header(header).is_err());
}

#[tokio::test]
async fn test_usage(){
    use dotenv;
    dotenv::dotenv().ok();
    let header = dotenv::var("HEADER").expect(
        "There's no header present in the ENV. Try adding `.env` file and adding variables to it.",
    );
    set_ident_header(header).unwrap();
    let weather = get_weather_at_location(48.8566, 2.3522).await;
    if let Err(ref e) = weather{
        dbg!(e);
        assert!(false);
    }
    assert!(weather.is_ok());
    dbg!(weather.unwrap());
}

#[tokio::test]
async fn test_request(){
    use dotenv;
    dotenv::dotenv().ok();
    let header = dotenv::var("HEADER").expect(
        "There's no header present in the ENV. Try adding `.env` file and adding variables to it.",
    );
    set_ident_header(header).unwrap();
    let url = "https://api.met.no/weatherapi/locationforecast/2.0/complete.json?lat=48.8566&lon=2.3522";
    match get_weather_query(&url) {
        Ok(request) => {
            let response = request.send().await.unwrap().text().await.unwrap();
            println!("response:{}", response);
            assert!(true);
        }
        Err(e) => {
            println!("error: {}", e);
            assert!(false);
        }
    }
}

#[test]
fn generate_url(){
    let url = format!(, 48.123456789, 2.345343453);
    dbg!(url);
}