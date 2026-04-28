struct Todo {
    title: String,
    completed: bool,
}

impl Todo {
    fn new(title: &str) -> Todo {
        Todo {
            title: String::from(title),
            completed: false,
        }
    }

    fn mark_completed(&mut self) {
        self.completed = true;
    }

    fn display(&self) {
        let status = if self.completed { "done" } else { "open" };
        println!("[{}] {}", status, self.title);
    }
}

fn main() {
    let mut todo = Todo::new("Learn Rust");
    todo.display();

    todo.mark_completed();
    todo.display();
}