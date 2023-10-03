mod calculator;
mod process_args;
mod global;

fn main() {
    let result = process_args::process_args();
    println!("{}", result.message);
}

