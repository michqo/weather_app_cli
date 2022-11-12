use colored::*;
use std::process::exit;

use parse::parse;

mod parse;
mod utils;

static URL: &str = "http://weather-uno.fly.dev/";

static HELP: &str = "\
Weather app cli
USAGE:
 help       Shows this help message
 today      Shows average of today temps
 yesterday  Shows average of yesterday temps
 week       Shows average of past 7 days temps
 day        Shows average for the given day of month";

fn main() {
    if parse().is_err() {
        println!("{}", "error fetching the server".red().bold());
        exit(1);
    }
}
