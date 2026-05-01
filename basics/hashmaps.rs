use std::collections::HashMap;

struct Todo {
    id: u64,
    title: String,
    completed: bool,
}

fn main() {
    let mut todos: HashMap<u64, Todo> = HashMap::new();

    todos.insert(
        1,
        Todo {
            id: 1,
            title: String::from("Learn Rust"),
            completed: false,
        },
    );

    todos.insert(
        2,
        Todo {
            id: 2,
            title: String::from("Build Orchestra"),
            completed: false,
        },
    );

    match todos.get(&1) {
        Some(todo) => println!("Found: {}", todo.title),
        None => println!("Not found"),
    }

    match todos.get_mut(&2) {
        Some(todo) => {
            todo.completed = true;
        }
        None => println!("Not found"),
    }

    for (id, todo) in &todos { // borrow the whole map to iterate over it without taking ownership of the todos
        let status = if todo.completed { "done" } else { "open" };
        println!("{}: {} [{}]", id, todo.title, status);
    }

    todos.insert(
        3,
        Todo {
            id: 3,
            title: String::from("Write benchmarks"),
            completed: false,
        },
    );

    match todos.get_mut(&3) {
        Some(todo) => { todo.completed = true; },
        None => println!("Not found"),
    }

    todos.remove(&1);
    for (id, todo) in &todos { 
        let status = if todo.completed { "done" } else { "open" };
        println!("{}: {} [{}]", id, todo.title, status);
    }

}