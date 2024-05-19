pub struct Task {
    pub name: String,
    pub task: Vec<String>,
}

impl Task {
    pub fn create_task(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Task, &'static str> {
        args.next();

        let name = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get input"),
        };

        let task: Vec<String> = args.collect();

        if task.is_empty() {
            return Err("Didn't get any tasks");
        }

        Ok(Task {
            name,
            task
        })
    }
}

pub fn run(task: Task) {
    match Task::create_task(task) {
        Ok(task) => {
            println!("Task name: {}", task.name);
            println!("Tasks:");
            for t in task.task {
                println!("- {}", t);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
} 