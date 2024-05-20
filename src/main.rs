use clap::{Arg, Command};
use task_scheduler_cli::TaskManager;

const DATA_FILE: &str = "tasks.json";

fn main() {
    let matches = Command::new("Task Manager")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Manages tasks")
        .subcommand(
            Command::new("add")
                .about("Adds a new task")
                .arg(Arg::new("name")
                    .help("Name of the task")
                    .required(true))
                .arg(Arg::new("task")
                    .help("Task details")
                    .required(true)
                    .num_args(1..)) // allows one or more arguments
        )
        .subcommand(Command::new("list").about("Lists all tasks"))
        .subcommand(
            Command::new("get")
                .about("Gets a specific task")
                .arg(Arg::new("name")
                    .help("Name of the task to get")
                    .required(true))
        )
        .get_matches();

    let mut task_manager = TaskManager::load_from_file(DATA_FILE).expect("Failed to load tasks");

    if let Some(matches) = matches.subcommand_matches("add") {
        let name = matches.get_one::<String>("name").unwrap();
        let task_details: Vec<String> = matches.get_many::<String>("task")
            .unwrap_or_default()
            .map(|s| s.to_string())
            .collect();
        task_manager.add_task(name.to_string(), task_details);
        task_manager.save_to_file(DATA_FILE).expect("Failed to save tasks");
        println!("Task '{}' added successfully.", name);
    } else if let Some(_) = matches.subcommand_matches("list") {
        let mut tasks: Vec<_> = task_manager.list_tasks().iter().collect();
        tasks.sort_by_key(|(name, _)| name.to_lowercase());

        for (name, details) in tasks {
            println!("Task- {}:", capitalize_first_letter(name));
            for detail in details {
                println!("    - {}", detail);
            }
        }
    } else if let Some(matches) = matches.subcommand_matches("get") {
        let name = matches.get_one::<String>("name").unwrap();
        match task_manager.get_task(name) {
            Some(task) => {
                println!("Task- {}:", capitalize_first_letter(name));
                for detail in task {
                    println!("    - {}", detail);
                }
            },
            None => println!("No task found with name '{}'", name),
        }
    }
}

fn capitalize_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}