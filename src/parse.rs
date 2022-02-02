use chrono::prelude::*;
use chrono::Duration;
use colored::*;
use serde::Deserialize;
use std::{env, process::exit};

#[allow(non_snake_case, dead_code)]
#[derive(Deserialize)]
struct Temp {
    pub y: i32,
    pub m: i32,
    pub d: i32,
    pub h: i32,
    pub averageTemp: String,
}

static URL: &str = "http://192.168.0.100:8080/";

fn average(temps: &[Temp]) -> f32 {
    temps
        .iter()
        .fold(0.0, |a, t| a + t.averageTemp.parse::<f32>().unwrap())
        / temps.len() as f32
}

fn fetch_temps(month: String, day: String) -> Result<Vec<Temp>, minreq::Error> {
    let response = minreq::get(format!("{}temps/{}/{}", URL, month, day)).send()?;
    response.json()
}

pub fn parse() -> Result<(), minreq::Error> {
    let mut args: Vec<String> = env::args().collect();
    args.drain(0..1);
    let last_url = format!("{}last_temp", URL);
    let arg = match args.get(0) {
        Some(s) => s.as_str(),
        _ => {
            let response = minreq::get(last_url).send()?;
            let json: Temp = response.json()?;
            println!(
                "{}: {}",
                "Last".cyan(),
                format!("{}°C", json.averageTemp).blue().bold()
            );
            return Ok(());
        }
    };
    match arg {
        "yesterday" => {
            let yesterday = Utc::now() - Duration::days(1);
            let temps = fetch_temps(yesterday.month().to_string(), yesterday.day().to_string())?;
            println!(
                "{}: {}",
                "Yesterday".cyan(),
                format!("{:.2}°C", average(&temps)).blue().bold()
            );
        },
        "day" => {
            let day = match args.get(1) {
                Some(s) => s,
                _ => {
                    println!("{}", "error: missing argument".red().bold());
                    exit(1);
                }
            };
            let temps = fetch_temps(Utc::now().month().to_string(), day.to_string())?;
            println!(
                "{}: {}",
                "Temp".cyan(),
                format!("{:.2}°C", average(&temps)).blue().bold()
            );
        },
        _ => {
            println!("{}", "error: invalid argument".red().bold());
            exit(1);
        }
    }
    Ok(())
}
