mod calculator;
mod process_args;
mod error;
mod response;
mod process_img;
use colored::Colorize;

fn main() {
    let result = process_args::process_args();
    if result.succeed {
        println!("{}", result.message.blue());
    }
    else {
        eprintln!("{}", result.message.red())
    }
}

