use chrono::prelude::*;
use colored::*;
use serde::Deserialize;
use std::env;

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
    let t = Utc::now().date();
    match arg {
        "yesterday" => {
            let response =
                minreq::get(format!("{}temps/{}", URL, t.day() - 1)).send()?;
            let json: Vec<Temp> = response.json()?;
            let average = json
                .iter()
                .fold(0.0, |a, t| a + t.averageTemp.parse::<f32>().unwrap())
                / json.len() as f32;
            println!(
                "{}: {}",
                "Yesterday".cyan(),
                format!("{:.2}°C", average).blue().bold()
            );
        }
        _ => (),
    }
    Ok(())
}
