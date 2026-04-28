struct Todo {
      id: u64,
      title: String,
      completed: bool,
}

fn find_todo(todos: &[Todo], id: u64) -> Option<&Todo> {
    for todo in todos {
        if todo.id == id {
            return Some(todo);
        }
    }
    None
}

fn main() {
    let todos = vec![
        Todo {
            id: 1,
            title: String::from("Learn Rust"),
            completed: false,
        },
        Todo {
            id: 2,
            title: String::from("Build Orchestra"),
            completed: false,
        },
    ];

    let result = find_todo(&todos, 2);

    match result {
        Some(todo) => println!("Found: {}", todo.title),
        None => println!("Todo not found"),
    }
}