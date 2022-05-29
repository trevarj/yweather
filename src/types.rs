use clap::Parser;
use serde::Deserialize;
use serde::Deserializer;

#[derive(Debug, Parser)]
#[clap(author, version)]
pub struct Opts {
    /// Location Longitude
    #[clap(long)]
    pub long: f32,

    /// Location Latitude
    #[clap(long)]
    pub lat: f32,

    /// Yandex.Weather API key
    #[clap(short, long)]
    pub key: String,
}

#[derive(Debug, Deserialize)]
pub struct WeatherData {
    pub fact: Fact,
}

#[derive(Debug, Deserialize)]
pub struct Fact {
    pub temp: f32,
    #[serde(deserialize_with = "condition_to_icon")]
    pub condition: String,
}

fn condition_to_icon<'de, D>(d: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = serde::de::Deserialize::deserialize(d)?;
    Ok(match s.as_str() {
        "clear" => " ",
        "partly-cloudy" => " ",
        "cloudy" => " ",
        "overcast" => " ",
        "drizzle" => " ",
        "light-rain" => " ",
        "rain" => " ",
        "moderate-rain" => " ",
        "heavy-rain" => " ",
        "continuous-heavy-rain" => " ",
        "showers" => " ",
        "wet-snow" => " ",
        "light-snow" => " ",
        "snow" => " ",
        "snow-showers" => " ",
        "hail" => " ",
        "thunderstorm" => " ",
        "thunderstorm-with-rain" => " ",
        "thunderstorm-with-hail" => " ",
        _ => "",
    }
    .to_string())
}
