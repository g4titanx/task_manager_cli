use clap::{builder::Str, Arg, Command};
use rustyline::{DefaultEditor, Editor};
use task_manager_cli::TaskManager;

const DATA_FILE: &str = "tasks.json";

fn main() {

    

    let matches = Command::new("Task Manager")
        .version("1.0")
        .author("g4titan, g4titan1@gmail.com")
        .about("Manages tasks")
        .subcommand(
            Command::new("add")
                .about("Adds a new task")
                .arg(Arg::new("name").help("Name of the task").required(true))
                .arg(
                    Arg::new("task")
                        .help("Task details")
                        .required(true)
                        .num_args(1..),
                ),
        )
        .subcommand(Command::new("list").about("Lists all tasks"))
        .subcommand(
            Command::new("get").about("Gets a specific task").arg(
                Arg::new("name")
                    .help("Name of the task to get")
                    .required(true),
            ),
        )
        .subcommand(
            Command::new("delete").about("Deletes a specific task").arg(
                Arg::new("name")
                    .help("Name of the task to delete")
                    .required(true),
            )
      
        )
        .subcommand(
            Command::new("update").about("updates a specific task").arg(
                Arg::new("name")
                    .help("Update objectives of a task")
                    .required(true),
            )
            .arg(
                    Arg::new("objective_id")
                        .help(" index of objective you wish to update ")
                        .required(true)
                )
        )
        .get_matches();

    let mut task_manager = TaskManager::load_from_file(DATA_FILE).expect("Failed to load tasks");

    if let Some(matches) = matches.subcommand_matches("add") {
        let name = matches.get_one::<String>("name").unwrap();
        let task_details: Vec<String> = matches
            .get_many::<String>("task")
            .unwrap_or_default()
            .map(|s| s.to_string())
            .collect();
        task_manager.add_task(name.to_string(), task_details);
        task_manager
            .save_to_file(DATA_FILE)
            .expect("Failed to save tasks");
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
            }
            None => println!("No task found with name '{}'", name),
        }
    } else if let Some(matches) = matches.subcommand_matches("delete") {
        let name = matches.get_one::<String>("name").unwrap();
        if task_manager.delete_task(name) {
            task_manager
                .save_to_file(DATA_FILE)
                .expect("Failed to save tasks");
            println!("Task '{}' deleted successfully.", name);
        } else {
            println!("No task found with name '{}'", name);
        }
    }
    else if let Some(matches) = matches.subcommand_matches("update") {
        let name = matches.get_one::<String>("name").unwrap();
        let objective_id_str = matches.get_one::<String>("objective_id").unwrap(); 
        let index = objective_id_str.parse::<usize>().expect("Error: objective_id must be a valid u8");
        let mut updated_data = String::new();
        match task_manager.get_mut_task(name) { 
        Some(task) => { 
   
        if (index) < task.len() {
        let current_value = &task[index];
        println!("Current objective value: {}", current_value);
        let mut rl = DefaultEditor::new().expect("Error initializing line editor");
        let prompt = format!("Edit objective [{}]: ", current_value);
        match rl.readline_with_initial(&prompt, (current_value, "")) {
        Ok(edited_value) => {
        task[index] = edited_value.to_string();
        updated_data  = edited_value;

        }
        Err(err) => {
                    println!("Error: {}", err);
        }
        }

            println!("Updated Task at '{}': {:?}", index, task);
        } else {
            println!("Error: Index {} is out of bounds for Vec at key '{}'", index, name);
        }
        task_manager.update_task(index, name, updated_data);
        task_manager
                .save_to_file(DATA_FILE)
                .expect("Failed to save tasks");
            }
            None => println!("No task found with name '{}'", name),
        }
    }


    // if let Some(task) = task_manager.get_task(name)
}

fn capitalize_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
