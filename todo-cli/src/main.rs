use std::env;
use std::error::Error;

#[derive(Debug)]
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
}

fn parse_command(args: &[String]) -> Result<Command, String> {
    if args.len() < 2 {
        return Err(String::from("expected a command"));
    }

    match args[1].as_str() {
        "add" => {
            if args.len() < 3 {
                return Err(String::from("expected a todo title"));
            }

            Ok(Command::Add(args[2].clone()))
        }
        "list" => Ok(Command::List),
        "done" => {
            if args.len() < 3 {
                return Err(String::from("expected todo id"));
            }

            let id = parse_id(&args[2])?;
            Ok(Command::Done(id))
        }
        "remove" => {
            if args.len() < 3 {
                return Err(String::from("expected todo id"));
            }

            let id = parse_id(&args[2])?;
            Ok(Command::Remove(id))
        }
        other => Err(format!("unknown command: {}", other)),
    }
}

fn parse_id(input: &str) -> Result<u64, String> {
    input
        .parse::<u64>()
        .map_err(|_| String::from("todo id must be a number"))
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let command = parse_command(&args)?;

    match command {
        Command::Add(title) => println!("Add: {}", title),
        Command::List => println!("List todos"),
        Command::Done(id) => println!("Done: {}", id),
        Command::Remove(id) => println!("Remove: {}", id),
    }

    Ok(())
}
