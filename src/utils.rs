use chrono::prelude::*;
use chrono::Duration;
use colored::*;
use std::process::exit;

use crate::temp::Temp;
use crate::URL;

pub fn print_temp(t: String) -> ColoredString {
    format!("{}°C", t).blue().bold()
}

pub fn week_averages(temps: Vec<Temp>) -> Vec<f32> {
    let mut averages: Vec<f32> = Vec::new();
    let now = Utc::now();
    for i in 0..8 {
        let day = now - Duration::days(i);
        let day_temps = temps.iter().filter(|t| t.d == day.day() as i32);
        let count = day_temps.clone().count() as f32;
        averages
            .push(day_temps.fold(0.0, |a, t| a + t.averageTemp.parse::<f32>().unwrap()) / count);
    }
    averages
}

pub fn average(temps: &[Temp]) -> f32 {
    temps
        .iter()
        .fold(0.0, |a, t| a + t.averageTemp.parse::<f32>().unwrap())
        / temps.len() as f32
}

pub fn fetch_temps(month: String, day: String) -> Result<Vec<Temp>, minreq::Error> {
    let response = minreq::get(format!("{}temps/{}/{}", URL, month, day)).send()?;
    if let Ok(r) = response.as_str() {
        if r == "[]" {
            println!("{}", "temps not found".red().bold());
            exit(1);
        }
    }
    response.json()
}

pub fn fetch_week() -> Result<Vec<Temp>, minreq::Error> {
    let response = minreq::get(format!("{}last_days/8", URL)).send()?;
    if let Ok(r) = response.as_str() {
        if r == "[]" {
            println!("{}", "temps not found".red().bold());
            exit(1);
        }
    }
    response.json()
}
