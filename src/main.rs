#![allow(unused)]

use clap::{Args, Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

#[derive(Parser)]
#[command(author, version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// use an custom db
    #[arg(long)]
    db: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// display todos
    Show(Show),

    /// add a new todo
    Add(Add),

    /// remove an exist todo by id
    Remove(Remove),

    /// toggle an exist todo by id
    Toggle(Toggle),

    /// clear all todos
    Clear(Clear),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum TodoStatus {
    Completed,
    Pending,
}

#[derive(Args)]
struct Show {
    /// display todos only with this status
    #[arg(value_enum, long, short)]
    only: Option<TodoStatus>,
}

#[derive(Args, Debug)]
struct Add {
    todo: String,
}

#[derive(Args)]
struct Remove {
    id: TodoID,
}

#[derive(Args)]
struct Toggle {
    id: TodoID,
}

#[derive(Args)]
struct Clear {
    /// clear todos only with this status
    #[arg(value_enum, long, short)]
    only: Option<TodoStatus>,
}

type TodoID = u8;

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    id: TodoID,

    todo: String,

    is_complete: bool,
}

impl std::fmt::Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(
            f,
            "{}. {} [{}]",
            self.id,
            self.todo,
            String::from(if self.is_complete {
                "completed"
            } else {
                "pending"
            })
        );
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct TodoApp {
    last_id: TodoID,
    todos: HashMap<TodoID, Todo>,
}

impl TodoApp {
    fn new(db: Option<String>) -> TodoApp {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open(db.unwrap_or(String::from("db.json")))
            .unwrap();

        match serde_json::from_reader(f) {
            Ok(todo_app) => {
                return todo_app;
            }
            Err(_) => {
                let todo_app_json = r#"{ "last_id": 0, "todos": {} }"#;

                return serde_json::from_str(todo_app_json).unwrap();
            }
        }
    }

    fn save(self, db: Option<String>) {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(db.unwrap_or(String::from("db.json")))
            .unwrap();

        serde_json::to_writer_pretty(f, &self).unwrap();
    }

    fn add(&mut self, todo: String) {
        self.last_id += 1;

        self.todos.insert(
            self.last_id,
            Todo {
                id: self.last_id,
                todo: todo,
                is_complete: false,
            },
        );
    }

    fn remove(&mut self, id: &TodoID) {
        self.todos.remove(id);
    }

    fn clear(&mut self, only: Option<TodoStatus>) {
        match only {
            Some(TodoStatus::Completed) => {
                self.todos.retain(|_, todo| !todo.is_complete);
            }

            Some(TodoStatus::Pending) => {
                self.todos.retain(|_, todo| todo.is_complete);
            }

            None => {
                self.todos.clear();
            }
        }
    }

    fn toggle(&mut self, id: &TodoID) {
        let mut todo = self.todos.get_mut(id).unwrap();

        todo.is_complete = !todo.is_complete;
    }

    fn show(&self, only: Option<TodoStatus>) {
        match only {
            Some(TodoStatus::Completed) => {
                for (id, todo) in &self.todos {
                    if todo.is_complete {
                        println!("{}", todo);
                    }
                }
            }

            Some(TodoStatus::Pending) => {
                for (id, todo) in &self.todos {
                    if !todo.is_complete {
                        println!("{}", todo);
                    }
                }
            }

            None => {
                for (id, todo) in &self.todos {
                    println!("{}", todo);
                }
            }
        }
    }
}

fn main() {
    let args = Cli::parse();
    let mut todo_app = TodoApp::new(args.db.clone());

    match &args.command {
        Commands::Add(options) => {
            todo_app.add(options.todo.clone());
            todo_app.save(args.db);
        }

        Commands::Remove(options) => {
            todo_app.remove(&options.id);
            todo_app.save(args.db);
        }

        Commands::Clear(options) => {
            todo_app.clear(options.only);
            todo_app.save(args.db);
        }

        Commands::Toggle(options) => {
            todo_app.toggle(&options.id);
            todo_app.save(args.db);
        }

        Commands::Show(options) => {
            todo_app.show(options.only);
        }
    }
}
