use std::io;
use serde::Deserialize;
use colored::*;

// Struct to deserialize the JSON response from openWeatherMap API
#[derive(Deserialize, Debug)]
struct WeatherResponse{
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
}

// Struct to represent weather description
#[derive(Deserialize, Debug)]
struct Weather{
    description: String,
}

// Struct to represent the main weather parameters
#[derive(Deserialize, Debug)]
struct Main{
    temp: f64,
    humidity: f64,
    pressure: f64,
}

// Struct to represent wind Information
#[derive(Deserialize, Debug)]
struct Wind{
    speed: f64,
}

// Function to get the weather information from OpenWeatherMap API
fn get_weather_info(city: &str, country_code: &str, api_key: &str) -> Result<WeatherResponse, reqwest::Error>{
    let url: String = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}", city, country_code, api_key
    );
    let response = reqwest::blocking::get(&url)?;
    let response_json: WeatherResponse = response.json::<WeatherResponse>()?;
    Ok(response_json)
}

// Function to display the weather information
fn display_weather_info(response: &WeatherResponse) {
    // Extract the weather information from the response
    let description = &response.weather[0].description;
    let temperature = response.main.temp;
    let humidity = response.main.humidity;
    let pressure = response.main.pressure;
    let wind_speed = response.wind.speed;
    
    // Display the weather information
    let weather_text: String = format!(
        "Weather in {}: {} {}
        > Temperature: {:.1}°C,
        > Humidity: {:.1}%,
        > Pressure: {:.1} hPa,
        > Wind Speed: {:.1} m/s,",
        response.name, description, get_temp_emoji(temperature), temperature, humidity, pressure, wind_speed
    );

    // Colorig the weather text based on temperature
    let weather_text_colored: ColoredString = match description.as_str() {
        "clear sky" => weather_text.bright_yellow(),
        "few clouds" => weather_text.bright_cyan(),
        "scattered clouds" => weather_text.bright_white(),
        "broken clouds" => weather_text.bright_black(),
        "shower rain" => weather_text.bright_blue(),
        "rain" => weather_text.bright_blue(),
        "thunderstorm" => weather_text.bright_blue(),
        "snow" => weather_text.bright_white(),
        "mist" => weather_text.bright_white(),
        _ => weather_text.normal(),
    };

    // Print the weather information
    println!("{}", weather_text_colored);
}

// Function to get emoji based on temperature
fn get_temp_emoji(temperature: f64) -> &'static str {
    if temperature < 0.0 {
        "❄️"
    } else if temperature >= 0.0 && temperature < 15.0 {
        "☁️"
    } else if temperature >= 15.0 && temperature < 25.0 {
        "🌤️"
    } else if temperature >= 25.0 && temperature < 35.0 {
        "☀️"
    } else {
        "🔥"
    }
}

fn main() {
    println!("{}", "Welcome to the Weather CLI".bright_magenta());
    loop{
        // Reading City Name
        println!("{}", "Please Enter the name of the City You want to search for:".bright_green());
        let mut city = String::new();
        io::stdin().read_line(&mut city).expect("Failed to read input!");
        let city = city.trim();

        // Reading Country Code
        println!("{}", "Please Enter the country code (e.g., US, UK, PK):".bright_green());
        let mut country_code = String::new();
        io::stdin().read_line(&mut country_code).expect("Failed to read input!");
        let country_code = country_code.trim();

        // API key
        let api_key = "8e08eae5ba61c175a304fec6e95da472";

        // Calling the function to fetch weather information
        match get_weather_info(&city, &country_code, api_key){
        Ok(response) => {
            display_weather_info(&response); // Displaying Weather information
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
        }

        println!("{}", "Do You want to search for another city? (y/n)");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input!");
        let input = input.trim().to_lowercase();

        if input != "y" && input != "yes" {
            println!("{}", "Thank you for using the Weather CLI".bright_magenta());
            break;
        }
    }
}
