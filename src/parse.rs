use chrono::prelude::*;
use chrono::Duration;
use colored::*;
use std::{env, process::exit};

use crate::URL;
use crate::utils::*;
use crate::temp::Temp;

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
                print_temp(json.averageTemp)
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
        }
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
                "{} average: {}",
                format!("{}.{}.", day, Utc::now().month()).bright_cyan(),
                format!("{:.2}°C", average(&temps)).blue().bold()
            );
        }
        "today" => {
            let now = Utc::now();
            let temps = fetch_temps(now.month().to_string(), now.day().to_string())?;
            println!(
                "{}: {}",
                "Average".cyan(),
                format!("{:.2}°C", average(&temps)).blue().bold()
            );
        }
        /*"week" => {
            let now = Utc::now();
            let mut averages: Vec<f32> = Vec::new();
            for i in 0..8 {
                let day = now - Duration::days(i);
                let temps = fetch_temps(day.month().to_string(), day.day().to_string())?;
                let average = average(&temps);
                println!(
                    "{} average: {}",
                    format!("{}.{}.", day.day(), day.month()).bright_cyan(),
                    format!("{:.2}°C", average).blue().bold()
                );
                averages.push(average);
            }
            let week_ago = averages[averages.len() - 1];
            let today = averages[0];
            let percentage = today.max(week_ago) / today.min(week_ago) * 100.0 - 100.0;
            if week_ago > today {
                println!("{} colder than a week ago", format!("-{:.2}%", percentage).blue());
            } else {
                println!("{} warmer than a week ago", format!("+{:.2}%", percentage).yellow());
            }
        }*/
        "week" => {
            let temps = fetch_week()?;
            let averages = week_averages(temps);
            let now = Utc::now();
            for i in 0..8 {
                let day = now - Duration::days(i);
                let average = averages[i as usize];
                println!(
                    "{} average: {}",
                    format!("{}.{}.", day.day(), day.month()).bright_cyan(),
                    format!("{:.2}°C", average).blue().bold()
                );
            }
            let week_ago = averages[averages.len() - 1];
            let today = averages[0];
            let percentage = today.max(week_ago) / today.min(week_ago) * 100.0 - 100.0;
            if week_ago > today {
                println!("{} colder than a week ago", format!("-{:.2}%", percentage).blue());
            } else {
                println!("{} warmer than a week ago", format!("+{:.2}%", percentage).yellow());
            }
        }
        _ => {
            println!("{}", "error: invalid argument".red().bold());
            exit(1);
        }
    }
    Ok(())
}
