use reqwest::blocking::Client;
use serde::Deserialize;
use std::{
    env,
    io::{self, Write},
};

#[derive(Deserialize, Debug)]
pub struct WeatherData {
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
}

#[derive(Deserialize, Debug)]
pub struct Weather {
    description: String,
}
#[derive(Deserialize, Debug)]
pub struct Main {
    temp: f64,
    pressure: i64,
    humidity: i32,
}
#[derive(Deserialize, Debug)]
pub struct Wind {
    speed: f64,
}

pub fn weather_info(city: &str) {
    let key =
        env::var("weather_key").expect("Please set your API key in the environment variable ");

    let open_weather_api = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}",
        city, key
    );

    let client = Client::new();
    let resp = match client.get(&open_weather_api).send() {
        Ok(response) => response,
        Err(_) => return eprintln!("Unable to fetch data"),
    };
    let resp = match resp.text() {
        Ok(text) => text,
        Err(_) => return eprintln!("Unable to read response text"),
    };
    let data: WeatherData = match serde_json::from_str(&resp) {
        Ok(data) => data,
        Err(_) => return eprintln!("unable to deserialize data"),
    };

    println!("\nWeather in {}: {}\n> Temperature: {} \u{2103}\n> Pressure: {} hPa\n> Humidity: {}%\n> Wind Speed: {} m/s",
        city.to_uppercase(),
        data.weather.first().unwrap().description,
        (data.main.temp -273.15).round(),
        data.main.pressure,
        data.main.humidity,
        data.wind.speed
    );
}

pub fn prompt(prompt: &str) -> String {
    loop {
        println!("{}", prompt);
        io::stdout().flush().expect("failed to flush stdout");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("failed to read input");

        if input.trim().is_empty() {
            println!("This field must not be empty. Please enter a value ");
            continue;
        }
        return input.trim().to_string();
    }
}

pub fn clr() {
    if cfg!(target_os = "windows") {
        std::process::Command::new("cls").status().unwrap();
    } else {
        std::process::Command::new("clear").status().unwrap();
    }
}
