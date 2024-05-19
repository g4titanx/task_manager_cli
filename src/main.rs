use std::env;
use std::process;
use task_scheduler_cli::Task;

fn main() {
    let task = Task::create_task(env::args()).unwrap_or_else(|err| { 
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let E = run(task) {
        
    }
}