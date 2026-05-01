struct Todo {
    id: u64,
    title: String,
    completed: bool,
}

fn main() {
    let todos = vec![
        Todo {
            id: 1,
            title: String::from("Learn Rust"),
            completed: true,
        },
        Todo {
            id: 2,
            title: String::from("Build Orchestra"),
            completed: false,
        },
        Todo {
            id: 3,
            title: String::from("Write benchmarks"),
            completed: false,
        },
    ];

    let open_count = todos.iter().filter(|todo| !todo.completed).count();

    println!("Open todos: {}", open_count);

    let titles: Vec<&String> = todos.iter().map(|todo| &todo.title).collect(); //vector of borrowed references to the titles

    for title in titles {
        println!("{}", title);
    }

    let found = todos.iter().find(|todo| todo.id == 2);

    match found {
        Some(todo) => println!("Found: {}", todo.title),
        None => println!("Not found"),
    }

    let completed_count = todos.iter().filter(|todo| todo.completed).count();
    println!("Completed todos: {}", completed_count);
}
