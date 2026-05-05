trait Runnable {
    fn run(&self);
}

struct PrintTask {
    message: String,
}

impl Runnable for PrintTask {
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

struct AgentTask {
    name: String,
}

impl Runnable for AgentTask {
    fn run(&self) {
        println!("Running agent: {}", self.name);
    }
}

fn main() {
    let tasks: Vec<Box<dyn Runnable>> = vec![ // Using Box<dyn Runnable> to store different types of tasks in a single vector
        Box::new(PrintTask {
            message: String::from("Hello from a task"),
        }),
        Box::new(TodoTask {
            title: String::from("Learn dynamic traits"),
        }),
        Box::new(AgentTask {
            name: String::from("researcher"),
        }),
    ];

    for task in &tasks {
        task.run();
    }
}

/*
Box<T> puts a value on the heap and gives you an owned pointer to it
dyn Runnable means some type that implements Runnable, but it isnt known at compile time
Box<dyn Runnable> means “own a heap-allocated runnable thing.”
this is how somehting can hold mutliple tasks and call run on all of them 
*/