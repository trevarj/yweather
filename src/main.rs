use clap::Parser;
use types::Opts;
use types::WeatherData;

mod types;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::parse();

    let weather: WeatherData = ureq::get("https://api.weather.yandex.ru/v2/informers")
        .set("Access-Control-Allow-Headers", "X-Yandex-API-Key")
        .set("X-Yandex-API-Key", &opts.key)
        .query("lat", &opts.lat.to_string())
        .query("lon", &opts.long.to_string())
        .call()?
        .into_json()?;

    let temp = weather.fact.temp;
    let cond = weather.fact.condition;
    println!("{temp}° {cond}");
    Ok(())
}
