pub mod err;

// Реэкспортируем типы ошибок, чтобы они были доступны в тестах
pub use err::{ParseErr, ReadErr};

use std::error::Error;
use std::fs;

#[derive(Debug, Eq, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        // Чтение файла
        let content = fs::read_to_string(path).map_err(|e| {
            Box::new(err::ReadErr {
                child_err: Box::new(e),
            }) as Box<dyn Error>
        })?;

        // Парсинг JSON
        let parsed = json::parse(&content).map_err(|e| {
            Box::new(err::ParseErr::Malformed(Box::new(e))) as Box<dyn Error>
        })?;

        // Если список задач пуст или отсутствует
        if parsed["tasks"].is_empty() {
            return Err(Box::new(err::ParseErr::Empty));
        }

        // Извлечение заголовка и списка задач
        let title = parsed["title"].as_str().unwrap_or("").to_string();
        let mut tasks = Vec::new();

        for task in parsed["tasks"].members() {
            let id = task["id"].as_u32().unwrap_or(0);
            let description = task["description"].as_str().unwrap_or("").to_string();
            let level = task["level"].as_u32().unwrap_or(0);
            
            tasks.push(Task {
                id,
                description,
                level,
            });
        }

        Ok(TodoList { title, tasks })
    }
}
