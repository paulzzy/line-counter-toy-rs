use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    let input = env::args().nth(1).expect("no input");

    let reader: Box<dyn BufRead> = match input.as_ref() {
        "-" => Box::new(BufReader::new(io::stdin())),
        _ => Box::new(BufReader::new(File::open(&input).expect("file not found"))),
    };

    let count = reader.lines().count();

    println!("{}", count);
}
