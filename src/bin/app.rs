#![allow(dead_code, unused_imports)]


use dialoguer::{theme::ColorfulTheme, Input};
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
    // Result type for error handling
    // Since trait have unknown size, we need to wrap it in a Box
    
Ok(())
}