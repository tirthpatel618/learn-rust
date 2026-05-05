trait Runnable { // Define a trait with a method that must be implemented
    fn run(&self); // anything runnable must have a run method
}

struct PrintTask { 
    message: String,
}

struct AgentTask {
    name: String,
}

impl Runnable for PrintTask { // Implement the Runnable trait for PrintTask
    fn run(&self) {
        println!("{}", self.message);
    }
}

struct TodoTask {
    title: String,
}

impl Runnable for TodoTask {
    fn run(&self) {
        println!("Todo: {}", self.title);
    }
}

impl Runnable for AgentTask {
    fn run(&self) {
        println!("Running Agent: {}", self.name);
    }
}

fn execute(task: &impl Runnable) { // A function that takes any type that implements the Runnable trait
    task.run();
}

fn main() {
    let print_task = PrintTask {
        message: String::from("Hello from a task"),
    };

    let todo_task = TodoTask {
        title: String::from("Learn traits"),
    };

    let agent_task = AgentTask {
        name: String::from("researcher"),
    };

    execute(&print_task);
    execute(&todo_task);
    execute(&agent_task);
}

/*
struct defines data
impl Type defines methods direcrly on the type
Impl Trait for Type teaches a type how to satisfy that shared behavior


*/