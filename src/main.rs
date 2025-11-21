use std::{env, process};
use std::error::Error;
use std::fs;
use wichtele::{get_valid_assignment};

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

    let assignments = get_valid_assignment(&contents);
    println!("Final Assignments:");
    for assignment in &assignments {
        println!("{} {} to {} {}", assignment.person1.first_name, assignment.person1.last_name, assignment.person2.first_name, assignment.person2.last_name);
    }
    Ok(())
}

pub struct Config {
    pub file_path: String,
}
impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 1 {
            return Err("not enough arguments");
        }
        let file_path = args[1].clone();

        Ok(Config { file_path })
    }
}
