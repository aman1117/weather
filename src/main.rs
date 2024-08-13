use colored::*;
use serde::Deserialize;
use std::io;

#[derive(Deserialize, Debug)]
struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
}

#[derive(Deserialize, Debug)]
struct Weather {
    description: String,
}

#[derive(Deserialize, Debug)]
struct Main {
    temp: f32,
    humidity: f32,
    pressure: f32,
}

#[derive(Deserialize, Debug)]
struct Wind {
    speed: f32,
}

fn get_weather_info(
    city: &str,
    country_code: &str,
    api_key: &str,
) -> Result<WeatherResponse, reqwest::Error> {
    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={},{}&appid={}&units=metric",
        city, country_code, api_key
    );

    let response = reqwest::blocking::get(&url)?;
    let response_json: WeatherResponse = response.json::<WeatherResponse>()?;
    Ok(response_json)
}

fn display_weather_info(response: &WeatherResponse) {
    let description = &response.weather[0].description;
    let temperature = response.main.temp;
    let humidity = response.main.humidity;
    let pressure = response.main.pressure;
    let wind_speed = response.wind.speed;

    let weather_text = format!(
        "Weather in {}: {} {}
        > Temperature: {:.1}°C,
        > Humidity: {:.1}%,
        > Pressure: {:.1} hP,a
        > Wind Speed: {:.1} m/s",
        response.name,
        description,
        get_temp_emoji(temperature),
        temperature,
        humidity,
        pressure,
        wind_speed
    );

    let weather_text_colored = match description.as_str() {
        "clear sky" => weather_text.bright_yellow(),
        "few clouds" => weather_text.bright_blue(),
        "scattered clouds" => weather_text.bright_blue(),
        "broken clouds" => weather_text.bright_blue(),

        "overcast clouds" | "mist" | "haze" | "smoke" | "fog" | "sand" | "dust" | "squalls" => {
            weather_text.dimmed()
        }
        "shower rain" | "rain" | "thunderstorm" | "snow" | "tornado" => weather_text.bright_cyan(),

        _ => weather_text.normal(),
    };

    println!("{}", weather_text_colored);

    fn get_temp_emoji(temperature: f32) -> &'static str {
        if temperature < 0.0 {
            "❄️"
        } else if temperature < 10.0 {
            "🥶"
        } else if temperature < 20.0 {
            "😊"
        } else if temperature < 30.0 {
            "🌞"
        } else {
            "🔥"
        }
    }
}

fn main() {
    println!("{}", "Weather CLI".bright_yellow().bold());
    loop {
        println!("{}", "Enter the city name:".bright_green());

        let mut city = String::new();
        io::stdin()
            .read_line(&mut city)
            .expect("Failed to read line");
        let city: &str = city.trim();

        // country code
        println!(
            "{}",
            "Enter the country code (e.g., IN for India):".bright_green()
        );
        let mut country_code = String::new();
        io::stdin()
            .read_line(&mut country_code)
            .expect("Failed to read line");

        let country_code: &str = country_code.trim();
        let api_key = "6a0e42e701a56d1d141231333c903964";

        match get_weather_info(city, &country_code, api_key) {
            Ok(response) => {
                display_weather_info(&response);
            }
            Err(error) => {
                eprintln!("Error: {}", error);
            }
        }

        println!(
            "{}",
            "Do you want to check the weather for another city? (yes/no)".bright_green()
        );
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice: &str = choice.trim();

        if choice != "yes" {
            print!("{}", "Goodbye!\n".bright_yellow());
            break;
        }
    }
}
