#![allow(dead_code, unused_imports, unused_variables)]


use dialoguer::{theme::ColorfulTheme, Input};
use dotenv::dotenv;
use reqwest;
use serde::Deserialize;
use std::env;
use tokio;

#[derive(Deserialize, Debug)]
struct WeatherResponse {
    main: Main,
    weather: Vec<Weather>,
    name: String,
}

#[derive(Deserialize, Debug)]
struct Main {
    temp: f32,
    humidity: u32,
}


#[derive(Deserialize, Debug)]
struct Weather {
    description: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    dotenv().expect("Failed to load .env file");
    let api_key = env::var("OPENWEATHERMAP_API_KEY")
        .expect("➡️ API_KEY must be set in .env file");

    let client = reqwest::Client::new();

    loop {
        // Prompt user for city name using dialoguer
        let city: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter city name (or 'exit' to quit)")
            .validate_with(|input: &String| -> Result<(), &str> {
                if input.trim().is_empty() {
                    Err("City name cannot be empty")
                } else {
                    Ok(())
                }
            })
            .interact_text()?;

        // Exit condition
        if city.to_lowercase() == "exit" {
            println!("Goodbye!");
            break;
        }

        // Build the API URL
        let url = format!(
            "http://api.openweathermap.org/data/2.5/weather?q={}&appid={}",
            city, api_key
        );

        // Make the API request
        let res = client.get(&url).send().await?;

        // Check if the request was successful
        if res.status().is_success() {
            let weather_data: WeatherResponse = res.json().await?;

            // Convert temperature from Kelvin to Celsius
            let temp_celsius = weather_data.main.temp - 273.15;

            // Display the weather information
            println!("\nWeather in {}:", weather_data.name);
            println!("Temperature: {:.1}°C", temp_celsius);
            println!("Humidity: {}%", weather_data.main.humidity);
            println!(
                "Condition: {}",
                weather_data.weather[0].description
            );
        } else {
            println!("Error: Could not fetch weather data for {}.", city);
            println!("Status: {}", res.status());
        }

        println!("\n");
    }

Ok(())
}