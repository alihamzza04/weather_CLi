# Weather CLI

A simple command-line weather application written in Rust that fetches real-time weather data from the OpenWeatherMap API.

## Features

- Real-time weather information for any city worldwide
- Displays temperature, humidity, pressure, and wind speed
- Color-coded output based on weather conditions
- Temperature-based emoji indicators
- Interactive CLI with continuous search capability

## Prerequisites

- Rust (latest stable version)
- OpenWeatherMap API key

## Installation

1. Clone the repository:
```bash
git clone https://github.com/alihamzza04/weather_CLi.git
cd weather_CLi
```

2. Build the project:
```bash
cargo build --release
```

3. Run the application:
```bash
cargo run
```

Or run the binary directly:
```bash
./target/release/weather_cli
```

## Usage

1. Start the application
2. Enter the city name when prompted
3. Enter the country code (e.g., US, UK, PK, IN)
4. View the weather information displayed with color coding
5. Press `y` to search for another city or any other key to exit

## Example Output

```
Welcome to the Weather CLI
Please Enter the name of the City You want to search for:
Lahore
Please Enter the country code (e.g., US, UK, PK):
PK
Weather in Lahore: smoke 🌤️
        > Temperature: 22.0°C,
        > Humidity: 64.0%,
        > Pressure: 1008.0 hPa,
        > Wind Speed: 1.0 m/s,
Do You want to search for another city? (y/n)
```

## Dependencies

- `reqwest` - HTTP client for fetching weather data
- `serde` - JSON deserialization
- `colored` - Terminal color output

## API Key

The application uses OpenWeatherMap API. You'll need to obtain an API key from [OpenWeatherMap](https://openweathermap.org/api) and replace the key in the source code.

## License

MIT