use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum WeatherError{
    #[snafu(display("Request header not set!"))]
    RequestHeaderNotSet,
    #[snafu(display("Request header already set!"))]
    HeaderAlreadySet
}