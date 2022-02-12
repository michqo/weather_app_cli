use colored::*;
use std::process::exit;

use crate::URL;
use crate::temp::Temp;

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
