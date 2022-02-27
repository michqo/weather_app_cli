use colored::*;
use std::process::exit;

use parse::parse;

mod temp;
mod parse;
mod utils;

// static URL: &str = "http://192.168.0.100:8080/";
static URL: &str = "http://localhost:8080/";

fn main() {
    if parse().is_err() {
        println!("{}", "error fetching the server".red().bold());
        exit(1);
    }
}
