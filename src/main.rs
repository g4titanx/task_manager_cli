use clap::{App, Arg};
use task_scheduler_cli::TaskManager;

fn main() {
    let matches = App::new("Task Manager")
        .version("1.0")
        .author("g4titan g4titan1@gmail.com")
        .about("Manages tasks")

        .subcommand(App::new("add")
            .about("Adds a new task")
            .arg(Arg::with_name("name")
                .help("Name of the task")
                .required(true))
            .arg(Arg::with_name("task")
                .help("Task details")
                .required(true)
                .multiple(true)))

        .subcommand(App::new("list")
            .about("Lists all tasks"))

        .subcommand(App::new("get")
            .about("Gets a specific task")
            .arg(Arg::with_name("name")
                .help("Name of the task to get")
                .required(true)))
        .get_matches();

    let mut task_manager = TaskManager::new();

    if let Some(matches) = matches.subcommand_matches("add") {
        let name = matches.value_of("name").unwrap();

        let task_details: Vec<String> = matches.values_of("task").unwrap()
            .map(String::from)
            .collect();

        task_manager.add_task(name.to_string(), task_details);
        println!("Task '{}' added successfully.", name);
    } 
    
    else if let Some(_matches) = matches.subcommand_matches("list") {
        for (name, details) in task_manager.list_tasks() {
            println!("Task '{}': {:?}", name, details);
        }
    } 
    
    else if let Some(matches) = matches.subcommand_matches("get") {
        let name = matches.value_of("name").unwrap();
        match task_manager.get_task(name) {
            Some(task) => println!("Task '{}': {:?}", name, task),
            None => println!("No task found with name '{}'", name),
        }
    }
}