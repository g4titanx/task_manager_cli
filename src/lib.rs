use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TaskManager {
    tasks: HashMap<String, Vec<String>>,
}

impl TaskManager {
    pub fn new() -> TaskManager {
        TaskManager {
            tasks: HashMap::new(),
        }
    }

    pub fn add_task(&mut self, name: String, details: Vec<String>) {
        self.tasks.insert(name, details);
    }

    pub fn list_tasks(&self) -> &HashMap<String, Vec<String>> {
        &self.tasks
    }

    pub fn get_task(&self, name: &str) -> Option<&Vec<String>> {
        self.tasks.get(name)
    }
    
    pub fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        let file = File::create(filename)?;
        serde_json::to_writer(file, &self.tasks)?;
        Ok(())
    }

    pub fn load_from_file(filename: &str) -> std::io::Result<TaskManager> {
        let file = File::open(filename);
        match file {
            Ok(mut file) => {
                let mut content = String::new();
                file.read_to_string(&mut content)?;
                let tasks: HashMap<String, Vec<String>> = serde_json::from_str(&content)?;
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
        manager.add_task("TestTask".to_string(), vec!["Detail1".to_string(), "Detail2".to_string()]);
        assert_eq!(manager.get_task("TestTask"), Some(&vec!["Detail1".to_string(), "Detail2".to_string()]));
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