use colored::*;

pub fn print_temp(t: String) -> ColoredString {
    format!("{}°C", t).blue().bold()
}
