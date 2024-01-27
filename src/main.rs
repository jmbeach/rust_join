use clap::Parser;
use std::io::{self, BufRead};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(index=1, default_value_t = String::from(" "))]
    seperator: String,
}

fn main() {
    let args = Args::parse();
    // Get the delimiter from the command line arguments
    let delimiter = args.seperator;

    // Create a handle to read from standard input
    let stdin = io::stdin();
    let handle = stdin.lock();

    // Read lines from standard input and join them with the specified delimiter
    let joined = handle
        .lines()
        .map(|line| match line {
            Ok(line) => line,
            Err(_) => panic!("Failed to read from stdin"),
        })
        .collect::<Vec<String>>()
        .join(&delimiter);
    println!("{}", joined);
}
