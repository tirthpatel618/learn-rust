fn main() {
    println!("Hello, world!");
    let todo = Todo {
        id: 1,
        title: String::from("Learn Rust"),
        completed: false,
    };
    print_todo_title(&todo.title);
    println!("Item: {}", todo.title); // this will work because we are still the owner of the string)
}


struct Todo {
    id: u64,
    title: String,
    completed: bool,
}

fn print_todo_title(title: &str) {
    println!("Item: {}", title);
}