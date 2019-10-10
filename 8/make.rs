use std::env;
use std::io::{Error, ErrorKind};
use std::fs::{metadata, File};
use std::path::Path;
use std::io::{prelude::*, BufReader};
use std::collections::{HashMap, HashSet};
use std::process::exit;
use std::time::SystemTime;
use std::process::{Command,ExitStatus};

enum Status {
    Started,
    Done,
    Previous
}

struct Task {
    name: String,
    dependencies: HashSet<String>,
    steps: Vec<String>
}

impl Task {
    fn add_task(&mut self, task: String) {
        self.steps.push(task);
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut tasks = None;
    if args.len() > 1 {
        tasks = Some(&args[1..]);
    }

    let path = Path::new("Makefile");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why),
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

    if let Some(last) = last_task {
        declared_tasks.insert(last.name.to_string(), last);
    }

    if let Some(unwrapped_tasks) = tasks {
        let mut done: HashMap<String, Status> = HashMap::new();
        for task in unwrapped_tasks {
            run(&task, &declared_tasks, &mut done, None);
            for (key, value) in done.iter_mut() {
                *value = Status::Previous;
            }
        }
    } else {
        if let Some(default) = first_task {
            let mut done: HashMap<String, Status> = HashMap::new();
            run(&default, &declared_tasks, &mut done, None);
        } else {
            return Err(Error::new(ErrorKind::Other, "No tasks defined in file"));
        }
    }
    return Ok(());
}

fn run_step(step: &String) -> Option<ExitStatus> {
    println!("{}", step[1..].to_string());

    let mut split = step.split_whitespace();

    if let Some(first) = split.next() {
        let mut cmd = Command::new(first);
        for arg in split {
            cmd.arg(arg);
        }
        match cmd.output() {
            Ok(output) => {
                println!("{}", String::from_utf8_lossy(&output.stdout));
                return Some(output.status);
            },
            Err(output) => {
                return None;
            }
        }
    }
    return None;
}

fn run(task: &String, declared_tasks: &HashMap<String, Task>, done: &mut HashMap<String, Status>, cause: Option<&Task>) -> std::io::Result<()> {
    let found = declared_tasks.get(task);
    if let Some(found_task) = found {
        if should_run(found_task, declared_tasks, done, cause) { //as_ref?
            done.insert(task.to_string(), Status::Started);
            for dependency in found_task.dependencies.iter() {
                run(&dependency, declared_tasks, done, Some(found_task));
            }
            for step in found_task.steps.iter() {
                run_step(step);
            }
            done.insert(task.to_string(), Status::Done);
        }
    } else {
        return Err(Error::new(ErrorKind::Other, format!("Task {} not declared.\n", task)));
    }
    return Ok(());
}

fn should_run(task: &Task, declared_tasks: &HashMap<String, Task>, done: &mut HashMap<String, Status>, cause: Option<&Task>) -> bool {
    if let Some(caused_by) = cause {
        match done.get(&task.name) {
            Some(Status::Started) => {
                println!("{}: Circular {} <- {} dependency dropped.", env::args().next().unwrap(), task.name, caused_by.name);
                //fix dependencies on task
                return false;
            },
            Some(Status::Previous) => {
                println!("{} already up to date", task.name);
                return false;
            },
            default => {}
        }
    }

    let task_updated = get_updated(&task.name);

    if task.dependencies.len() == 0 {
        return true;
    }

    for dependency in task.dependencies.iter() {
        let dependency_updated = get_updated(dependency);

        if task_updated.is_ok() && dependency_updated.is_ok() {
            if task_updated.as_ref().unwrap() < dependency_updated.as_ref().unwrap() {
                return true;
            }
        } else {
            return true;
        }
    }

    return false;
}

fn get_updated(task: &String) -> std::io::Result<SystemTime> {
    let data = metadata(task)?;

    if let Ok(time) = data.modified() {
        Ok(time)
    } else {
        Err(Error::new(ErrorKind::Other, "Not supported on this platform"))
    }
}