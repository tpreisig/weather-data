
# Weather CLI Application

## Overview

This is a command-line interface (CLI) application written in Rust that fetches and displays current weather information for a specified city using the OpenWeatherMap API. The application prompts the user to enter a city name, retrieves weather data, and displays the temperature (in Celsius), humidity, and weather conditions. Users can exit the application by typing "exit".

## Features

- Interactive CLI with user input for city names
- Fetches real-time weather data using the OpenWeatherMap API
- Displays temperature (converted from Kelvin to Celsius), humidity, and weather description
- Error handling for invalid city names and API request failures
- Continuous loop for multiple queries until the user chooses to exit

## Prerequisites

To run this application, you need the following:
- [Rust](https://www.rust-lang.org/tools/install) (stable version recommended)
- An [OpenWeatherMap API key](https://openweathermap.org/api) (free tier available)
- A `.env` file with your API key (see Configuration below)

## Dependencies

The application uses the following Rust crates:
- `dialoguer`: For interactive CLI prompts
- `dotenv`: For loading environment variables from a `.env` file
- `reqwest`: For making HTTP requests to the OpenWeatherMap API
- `serde`: For deserializing JSON responses
- `tokio`: For asynchronous runtime support

These dependencies are specified in the `Cargo.toml` file.

## Installation

1. Clone or download this repository to your local machine.
2. Ensure Rust is installed by running:
   ```bash
   rustc --version
   ```
   If not installed, follow the [Rust installation guide](https://www.rust-lang.org/tools/install).
3. Navigate to the project directory:
   ```bash
   cd path/to/project
   ```
4. Install dependencies by running:
   ```bash
   cargo build
   ```

## Configuration

1. Create a `.env` file in the root of the project directory.
2. Add your OpenWeatherMap API key to the `.env` file:
   ```
   OPENWEATHERMAP_API_KEY=your_api_key_here
   ```
   Replace `your_api_key_here` with your actual API key from OpenWeatherMap.

## Usage

1. Run the application:
   ```bash
   cargo run
   ```
2. When prompted, enter a city name (e.g., "London" or "New York").
3. The application will display the current weather information, including:
   - City name
   - Temperature in Celsius
   - Humidity percentage
   - Weather condition (e.g., "clear sky", "light rain")
4. To query another city, enter a new city name.
5. To exit, type `exit` when prompted.

Example output:
```
Enter city name (or 'exit' to quit): London

Weather in London:
Temperature: 15.2°C
Humidity: 80%
Condition: light rain

Enter city name (or 'exit' to quit):
```

## Error Handling

- If the city name is empty, the application will prompt for a valid input.
- If the API request fails (e.g., invalid city or network issue), an error message with the HTTP status code will be displayed.
- If the `.env` file or API key is missing, the application will exit with an error.

## Notes

- The temperature is converted from Kelvin (provided by the API) to Celsius for display.
- The application uses the `tokio` runtime for asynchronous API requests.
- Ensure a stable internet connection for API requests to succeed.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

