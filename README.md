# YRNO

It's a library used to communicate and fetch weather data and forecasts from [yr.no](https://yr.no).

## Notes

- You need to get Google's API key to use the `location` module and set it using

```Rust
let location = Location::new();
location.set_api_key("<API_KEY>");
// Use your `geocode_location` queries below
if let Ok((lat, lon)) = geocode_location("Paris").await{
  // You have a `lat` and `lon` of Paris here.
}
```

- You need to set the API Request header to something that meets [yr.no's TOS](https://developer.yr.no/doc/TermsOfService/).
The header is set once per application runtime. If you try to set it multiple times, you'll get an error.

```Rust
fn main(){
  set_ident_header(String::from("<YOUR_HEADER>")).unwrap();
  // Now you can use `get_weather_at_location()`.
  let weather_in_paris = get_weather_at_location(48.8566, 2.3522).await;

  // If we try `set_ident_header` again, we'll get an error.
  set_ident_header(String::from("<YOUR_HEADER_2>")).unwrap_err();
}
```

## Roadmap

- [x] Geocode
- [ ] Weather
  - [x] CompleteWeather
  - [ ] CompactWeather
- [ ] Wind
- [ ] Water temperatures
