use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::io::{self, Write};

const TODO_FILE: &str = "todos.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
// Clone is needed to allow us to clone the todo when we mark it as done, since we need to modify it in place
// Serialize and Deserialize are needed to convert the todo to and from JSON when we save and load the todos
struct Todo {
    id: u64,
    title: String,
    completed: bool,
}

#[derive(Debug)]
enum Command {
    Add(String),
    List,
    Done(u64),
    Remove(u64),
    Help,
    Quit,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut todos = load_todos()?; // load existing todos from the file, if it exists, else start with an empty list

    println!("Todo CLI");
    print_help();

    loop { // Main command loop
        print!("todo> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        let command = match parse_command(input) {
            Ok(command) => command,
            Err(message) => {
                println!("Error: {}", message);
                continue;
            }
        };

        match command {
            Command::Add(title) => {
                add_todo(&mut todos, title);
                if let Err(error) = save_todos(&todos) {
                    println!("Error: failed to save todos: {}", error);
                }
            }
            Command::List => list_todos(&todos),
            Command::Done(id) => {
                if let Err(error) = mark_done(&mut todos, id) {
                    println!("Error: {}", error);
                    continue;
                }

                if let Err(error) = save_todos(&todos) {
                    println!("Error: failed to save todos: {}", error);
                }
            }
            Command::Remove(id) => {
                if let Err(error) = remove_todo(&mut todos, id) {
                    println!("Error: {}", error);
                    continue;
                }

                if let Err(error) = save_todos(&todos) {
                    println!("Error: failed to save todos: {}", error);
                }
            }
            Command::Help => print_help(),
            Command::Quit => break,
        }
    }

    Ok(())
}

fn parse_command(input: &str) -> Result<Command, String> {
    let mut parts = input.split_whitespace();
    let command = parts
        .next()
        .ok_or_else(|| String::from("expected a command"))?;

    match command {
        "add" => {
            let title = input
                .strip_prefix("add")
                .map(str::trim)
                .filter(|title| !title.is_empty())
                .ok_or_else(|| String::from("expected a todo title"))?;

            Ok(Command::Add(title.to_string()))
        }
        "list" => Ok(Command::List),
        "done" => {
            let id = parse_required_id(parts.next())?;
            Ok(Command::Done(id))
        }
        "remove" => {
            let id = parse_required_id(parts.next())?;
            Ok(Command::Remove(id))
        }
        "help" => Ok(Command::Help),
        "quit" | "exit" => Ok(Command::Quit),
        other => Err(format!("unknown command: {}", other)),
    }
}

fn parse_required_id(input: Option<&str>) -> Result<u64, String> {
    let raw_id = input.ok_or_else(|| String::from("expected todo id"))?;

    raw_id
        .parse::<u64>()
        .map_err(|_| String::from("todo id must be a number"))
}

fn load_todos() -> Result<Vec<Todo>, Box<dyn Error>> {
    match fs::read_to_string(TODO_FILE) {
        Ok(contents) => {
            let todos = serde_json::from_str::<Vec<Todo>>(&contents)?;
            Ok(todos)
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(Vec::new()),
        Err(error) => Err(Box::new(error)),
    }
}

fn save_todos(todos: &[Todo]) -> Result<(), Box<dyn Error>> {
    let contents = serde_json::to_string_pretty(todos)?;
    fs::write(TODO_FILE, contents)?;
    Ok(())
}

fn add_todo(todos: &mut Vec<Todo>, title: String) {
    let next_id = todos.iter().map(|todo| todo.id).max().unwrap_or(0) + 1;

    todos.push(Todo {
        id: next_id,
        title,
        completed: false,
    });

    println!("Added todo {}", next_id);
}

fn list_todos(todos: &[Todo]) {
    if todos.is_empty() {
        println!("No todos");
        return;
    }

    for todo in todos {
        let status = if todo.completed { "done" } else { "open" };
        println!("{}: {} [{}]", todo.id, todo.title, status);
    }
}

fn mark_done(todos: &mut [Todo], id: u64) -> Result<(), Box<dyn Error>> {
    let todo = todos
        .iter_mut()
        .find(|todo| todo.id == id)
        .ok_or_else(|| format!("todo {} not found", id))?;

    todo.completed = true;
    println!("Marked todo {} as done", id);
    Ok(())
}

fn remove_todo(todos: &mut Vec<Todo>, id: u64) -> Result<(), Box<dyn Error>> {
    let original_len = todos.len();
    todos.retain(|todo| todo.id != id);

    if todos.len() == original_len {
        return Err(format!("todo {} not found", id).into());
    }

    println!("Removed todo {}", id);
    Ok(())
}

fn print_help() {
    println!("Commands:");
    println!("  add <title>");
    println!("  list");
    println!("  done <id>");
    println!("  remove <id>");
    println!("  help");
    println!("  quit");
}
