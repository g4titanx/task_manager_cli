use clap::builder::Str;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, Read};

/// Represents a task manager that loads tasks from a JSON file.
#[derive(Serialize, Deserialize, Debug)]
pub struct TaskManager {
    tasks: IndexMap<String, Vec<String>>,
}

impl TaskManager {
    /// Creates a new `TaskManager`.
    pub fn new() -> Self {
        TaskManager {
            tasks: IndexMap::new(),
        }
    }

    /// Adds a new task with details.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the task.
    /// * `details` - A vector of details about the task.
    pub fn add_task(&mut self, name: String, details: Vec<String>) {
        self.tasks.insert(name, details);
    }

    /// Lists all tasks.
    pub fn list_tasks(&self) -> &IndexMap<String, Vec<String>> {
        &self.tasks
    }

    /// Gets a specific task by name.
    ///
    /// Returns `None` if the task does not exist.
    pub fn get_task(&self,  name: &str) ->  Option<&Vec<String>> {
        self.tasks.get(name)
    }
    /// Gets a mutable task by name 
    pub fn get_mut_task(&mut self,  name: &str) ->  Option<&mut Vec<String>> {
        self.tasks.get_mut(name)
    }
    /// Deletes a specific task by name.
    ///
    /// Returns `true` if the task was found and deleted, otherwise `false`.
    pub fn delete_task(&mut self, name: &str) -> bool {
    self.tasks.swap_remove(name).is_some()
    }
    /// Updates a specific task objective
    pub fn update_task(&mut self, index:usize, name: &str, updated_data: String) {
    match  self.tasks.get_mut(name){
    Some(objective) =>{
    objective[index] = updated_data;
    }  
    None=>{
    println!("data not found for index{}", index);
    }
    }}
    /// Saves the task manager to a JSON file.
    pub fn save_to_file(&self, filename: &str) -> io::Result<()> {
        let file = File::create(filename)?;
        serde_json::to_writer(file, &self.tasks)?;
        Ok(())
    }

    /// Loads the task manager from a JSON file.
    pub fn load_from_file(filename: &str) -> io::Result<Self> {
        let file = File::open(filename);
        match file {
            Ok(mut file) => {
                let mut content = String::new();
                file.read_to_string(&mut content)?;
                if content.is_empty() {
                    return Ok(TaskManager::new());
                }
                let tasks: IndexMap<String, Vec<String>> = serde_json::from_str(&content)?;
                Ok(TaskManager { tasks })
            }
            Err(_) => Ok(TaskManager::new()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_get_task() {
        let mut manager = TaskManager::new();
        manager.add_task(
            "TestTask".to_string(),
            vec!["Detail1".to_string(), "Detail2".to_string()],
        );
        assert_eq!(
            manager.get_task("TestTask"),
            Some(&vec!["Detail1".to_string(), "Detail2".to_string()])
        );
    }

    #[test]
    fn test_list_tasks() {
        let mut manager = TaskManager::new();
        manager.add_task("Task1".to_string(), vec!["Detail1".to_string()]);
        manager.add_task("Task2".to_string(), vec!["Detail2".to_string()]);
        let tasks = manager.list_tasks();
        assert!(tasks.contains_key("Task1"));
        assert!(tasks.contains_key("Task2"));
    }

    #[test]
    fn test_get_nonexistent_task() {
        let manager = TaskManager::new();
        assert!(manager.get_task("Nonexistent").is_none());
    }
}
