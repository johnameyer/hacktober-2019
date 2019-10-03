use std::env;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::{prelude::*, BufReader};
use std::collections::HashSet;
use std::process::exit;

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

    let mut first_task = None;

    let mut declared_tasks: HashSet<String> = HashSet::new();

    for line in reader.lines() {

        let unwrap = line.unwrap();

        if unwrap.len() == 0 {
            continue;
        }

        let ch = unwrap.chars().next().unwrap();
        match ch {
            '\t' => {
                let command: Vec<&str> = unwrap.split_whitespace().collect();
            }
            _ => {
                let split: Vec<&str> = unwrap.splitn(2, ':').collect();
                let task = split[0];
                
                let dependencies: Vec<&str> = split[1].split_whitespace().collect();
                
                declared_tasks.insert(task.to_string());

                if first_task.is_none() {
                    first_task = Some(task.to_string());
                }
            }
        }
    }
    if tasks.is_some() {
        for task in tasks.unwrap() {
            if declared_tasks.contains(task) {
                println!("{}", task);
            } else {
                println!("Task {} not declared.\n", task);
                exit(1);
            }
        }
    } else {
        println!("{}", first_task.expect("No tasks defined in file"));
    }
}