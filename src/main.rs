use colored::*;
use std::process::exit;

use parse::parse;

mod parse;

fn main() {
    if parse().is_err() {
        println!("{}", "error fetching the server".red().bold());
        exit(-1);
    }
}
