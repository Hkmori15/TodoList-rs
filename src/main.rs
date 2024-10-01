use std::fs;
use std::path::Path;
use serde::{Serialize, Deserialize};
use clap::{Parser, Subcommand};

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    id: usize,
    description: String,
    done: bool,
}

impl Todo {
    fn new(id: usize, description: String) -> Todo {
        Todo { id, description, done: false }
    }

    fn mark_done(&mut self) {
        self.done = true;
    }

    fn edit(&mut self, new_description: String) {
        self.description = new_description;
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct TodoList {
    todos: Vec<Todo>,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList { todos: Vec::new() }
    }

    fn add(&mut self, description: String) {
        let id = self.todos.len() + 1;
        let todo = Todo::new(id, description);
        self.todos.push(todo);
    }

    fn delete(&mut self, id: usize) {
        self.todos.retain(|todo| todo.id != id);
    }

    fn edit(&mut self, id: usize, new_description: String) {
        if let Some(todo) = self.todos.iter_mut().find(|todo| todo.id == id) {
            todo.edit(new_description);
        }
    }

    fn mark_done(&mut self, id: usize) {
        if let Some(todo) = self.todos.iter_mut().find(|todo| todo.id == id) {
            todo.mark_done();
        }
    }

    fn list(&self) {
        if self.todos.is_empty() {
            println!("No tasks available.");
        } else {
            for todo in &self.todos {
                println!("#{}: {} [{}]", todo.id, todo.description, if todo.done { "Done" } else { "Not done" });
            }
        }
    }

    fn search(&self, keyword: &str) {
        let results: Vec<&Todo> = self.todos.iter()
            .filter(|todo| todo.description.to_lowercase().contains(&keyword.to_lowercase()))
            .collect();

        if results.is_empty() {
            println!("No tasks found with keyword '{}'.", keyword);
        } else {
            for todo in results {
                println!("#{}: {} [{}]", todo.id, todo.description, if todo.done { "Done" } else { "Not done" });
            }
        }
    }

    fn filter_by_status(&self, done: bool) {
        let result: Vec<&Todo> = self.todos.iter()
            .filter(|todo| todo.done == done)
            .collect();

        if result.is_empty() {
            if done {
                println!("No completed tasks found.");
            } else {
                println!("No pending tasks found.");
            }
        } else {
            for todo in result {
                println!("#{}: {} [{}]", todo.id, todo.description, if todo.done { "Done" } else { "Not done" });
            }
        }
    }

    fn save(&self, filepath: &str) {
        let data = serde_json::to_string_pretty(&self).expect("Could not serialize data");
        fs::write(filepath, data).expect("Could not write to file");
        println!("Todos saved to {}", filepath);
    }

    fn load(filepath: &str) -> TodoList {
        if Path::new(filepath).exists() {
            let data = fs::read_to_string(filepath).expect("Could not read file");
            serde_json::from_str(&data).expect("Could not deserialize data")
        } else {
            TodoList::new()
        }
    }
}

#[derive(Parser)]
#[command(name = "TODO List")]
#[command(about = "Simple TODO List in Rust", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        description: String,
    },

    Delete {
        id: usize,
    },

    Edit {
        id: usize,
        description: String,
    },

    Done {
        id: usize,
    },

    List, Search {
        keyword: String,
    },

    Filter {
        #[arg(short, long)]
        done: bool,
        #[arg(short, long)]
        not_done: bool,
    },

    Save {
        filepath: String,
    },

    Load {
        filepath: String,
    },
}

fn main() {
    let cli = Cli::parse();
    let filepath = "todos.json";
    let mut todo_list = TodoList::load(filepath);


    match cli.command {
        Commands::Add { description } => {
            todo_list.add(description);
            todo_list.save(filepath);
            println!("Task added.");
        }

        Commands::Delete { id } => {
            todo_list.delete(id);
            todo_list.save(filepath);
            println!("Task deleted.");
        }

        Commands::Edit { id, description } => {
            todo_list.edit(id, description);
            todo_list.save(filepath);
            println!("Task edited.");
        }

        Commands::Done { id } => {
            todo_list.mark_done(id);
            todo_list.save(filepath);
            println!("Task marked as done.");
        }

        Commands::List => {
            todo_list.list();
        }

        Commands::Search { keyword } => {
            todo_list.search(&keyword);
        }

        Commands::Filter { done, not_done } => {
            if done {
                todo_list.filter_by_status(true);
            } else if not_done {
                todo_list.filter_by_status(false);
            }
        }

        Commands::Save { filepath } => {
            todo_list.save(&filepath);
        }

        Commands::Load { filepath } => {
            todo_list = TodoList::load(&filepath);
            todo_list.list();
        }
    }   
}
