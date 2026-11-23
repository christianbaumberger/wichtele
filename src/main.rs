use std::{env, process};
use std::error::Error;
use std::fs;
use wichtele::{get_valid_assignment, parse_previous_assignments};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    };
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let previous_assignments = if let Some(prev_path) = config.previous_assignment_path {
        let prev_contents = fs::read_to_string(prev_path)?;
        Some(parse_previous_assignments(&prev_contents))
    } else {
        None
    };

    let assignments = get_valid_assignment(&contents, previous_assignments.as_ref());
    println!("Final Assignments:");
    for assignment in &assignments {
        println!("{} {} to {} {}", assignment.person1.first_name, assignment.person1.last_name, assignment.person2.first_name, assignment.person2.last_name);
    }
    Ok(())
}

pub struct Config {
    pub file_path: String,
    pub previous_assignment_path: Option<String>,
}
impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        let file_path = args[1].clone();
        let previous_assignment_path = if args.len() >= 3 {
            Some(args[2].clone())
        } else {
            None
        };

        Ok(Config { file_path, previous_assignment_path })
    }
}
