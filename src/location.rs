use std::cell::RefCell;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct LocationResponse{
    results: Vec<LocationResponseEntry>,
}

#[derive(Deserialize, Debug)]
struct LocationResponseEntry{
    geometry: Geometry
}

#[derive(Deserialize, Debug)]
struct Geometry{
    location: CoordinateLocation
}

#[derive(Deserialize, Debug, Copy, Clone)]
struct CoordinateLocation{
    lat: f32,
    lng: f32,
}

#[derive(Default)]
pub struct Location{
    api_key: RefCell<String>,
}

impl Location{
    pub fn new() -> Self{
        Location{
            api_key: RefCell::new(String::new()),
        }
    }

    /// Sets the API key for Google Geocoding, returning the previous one.
    pub fn set_api_key(&self, key: String) -> String{
        self.api_key.replace(key)
    }

    /// Turns address into coordinates using Google's Geocoding API.
    pub async fn geocode_location(&self, location: &str) -> Result<Vec<(f32, f32)>, Box<dyn std::error::Error>>{
        let url = format!("https://maps.googleapis.com/maps/api/geocode/json?key={:.4}&address={:.4}", self.api_key.borrow(), location);
        let response: LocationResponse = reqwest::get(url).await?.json().await?;
        let data = response.results.iter().map(|a|{
            let location = a.geometry.location;
            (location.lat, location.lng)
        }).collect::<Vec<(f32, f32)>>();
        Ok(data)
    }
}

#[tokio::test]
async fn get_location(){
    use dotenv;
    dotenv::dotenv().ok();
    let key = dotenv::var("API_KEY").expect("There's no key present in the ENV. Try adding `.env` file and adding variables to it.");
    let location = Location::new();
    location.set_api_key(key.to_owned());
    println!("{:#?}",location.geocode_location("Paris").await.unwrap());
}