use std::env;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::{prelude::*, BufReader};
use std::collections::HashMap;
use std::process::exit;

struct Task {
    name: String,
    dependencies: Vec<String>,
    steps: Vec<String>
}

impl Task {
    fn add_task(&mut self, task: String) {
        self.steps.push(task);
    }
}

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

    let mut declared_tasks: HashMap<String, Task> = HashMap::new();

    let mut last_task: Option<Task> = None;

    for line in reader.lines() {

        let unwrap = line.unwrap();

        if unwrap.len() == 0 {
            continue;
        }

        let ch = unwrap.chars().next().unwrap();
        match ch {
            '\t' => {
                let command = unwrap.to_string();
                if last_task.is_some() {
                    last_task.as_mut().unwrap().add_task(command);
                }
            }
            _ => {
                let split: Vec<&str> = unwrap.splitn(2, ':').collect();
                let task = split[0];
                
                let dependencies: Vec<&str> = split[1].split_whitespace().collect();

                if last_task.is_some() {
                    let x = last_task.unwrap();
                    declared_tasks.insert(x.name.to_string(), x);    
                }

                last_task = Some(Task {
                    name: task.to_string(),
                    dependencies: dependencies.iter().map(|x| x.to_string()).collect(),
                    steps: Vec::new()
                });

                if first_task.is_none() {
                    first_task = Some(task.to_string());
                }
            }
        }
    }

    if last_task.is_some() {
        let last = last_task.unwrap();
        declared_tasks.insert(last.name.to_string(), last);
    }

    if tasks.is_some() {
        for task in tasks.unwrap() {
            let found = declared_tasks.get(task);
            if found.is_some() {
                println!("{}", task);
                for dependency in found.as_ref().unwrap().dependencies.iter() {
                    println!("{}", dependency);
                }
            } else {
                println!("Task {} not declared.\n", task);
                exit(1);
            }
        }
    } else {
        println!("{}", first_task.expect("No tasks defined in file"));
    } 
}