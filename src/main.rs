use colored::*;
use std::process::exit;

use parse::parse;

mod parse;
mod utils;

// static URL: &str = "http://192.168.0.100:8080/";
// static URL: &str = "http://localhost:8080/";
static URL: &str = "http://127.0.0.1:8000/";

static HELP: &str = "\
Weather app cli
USAGE:
 help       Shows this help message
 today      Shows average of today temps
 yesterday  Shows average of yesterday temps
 week       Shows average of past 7 days temps";

fn main() {
    if parse().is_err() {
        println!("{}", "error fetching the server".red().bold());
        exit(1);
    }
}
