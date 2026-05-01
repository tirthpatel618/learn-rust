struct Todo {
    id: u64,
    title: String,
    completed: bool,
}

fn main() {
    let mut todos: Vec<Todo> = Vec::new();

    todos.push(Todo {
        id: 1,
        title: String::from("Learn Rust"),
        completed: false,
    });

    todos.push(Todo {
        id: 2,
        title: String::from("Build Orchestra"),
        completed: false,
    });

    for todo in &todos { // Need a reference here to avoid moving the todo out of the vector
        println!("{}: {}", todo.id, todo.title);
    }

    for todo in &mut todos { // Mutable reference to modify the todos
        if todo.id == 1 {
            todo.completed = true; // Mark the first todo as completed
        }
    }

    let found = todos.iter().find(|todo| todo.id == 2); // searches without taking ownership of the todo

    match found {
        Some(todo) => println!("Found todo: {}", todo.title),
        None => println!("Todo not found"),
    }
}