use std::env;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::{prelude::*, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut tasks = None;
    if args.len() > 1 {
        tasks = Some(&args[1..]);
    }

    let path = Path::new("Makefile");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.description()),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);

    for line in reader.lines() {

        let unwrap = line.unwrap();
        println!("{}", unwrap);
    }
}