enum Command {
    Add(String),
    List,
    Done(u64),
    Remove(u64)
}

fn handle_command(cmd: Command) {
    match cmd {
        Command::Add(title) => println!("Adding todo: {}", title),
        Command::List => println!("Listing todos..."),
        Command::Done(id) => println!("Marking todo {} as done", id),
        Command::Remove(id) => println!("Removing todo {}", id),
    }
}

fn main() {
    let cmd1 = Command::Add(String::from("Learn Rust"));
    let cmd2 = Command::List;
    let cmd3 = Command::Done(1);
    let cmd4 = Command::Remove(1);

    handle_command(cmd1);
    handle_command(cmd2);
    handle_command(cmd3);
    handle_command(cmd4);
}