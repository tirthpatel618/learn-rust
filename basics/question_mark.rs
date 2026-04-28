fn parse_id(input: &str) -> Result<u64, String> {
    match input.parse::<u64>() {
        Ok(id) => Ok(id),
        Err(_) => Err(String::from("ID must be a positive number")),
    }
}


// this just removes boilerplate from the match
fn double_id(input: &str) -> Result<u64, String> {
    let id = parse_id(input)?;
    Ok(id * 2)
}

fn main() {
    let result = double_id("21");

    match result {
        Ok(value) => println!("Doubled ID: {}", value),
        Err(message) => println!("Error: {}", message),
    }
}

//   This will matter a lot for file I/O:
// let contents = std::fs::read_to_string("todos.json")?;
// either gives you the contents or returns the erorr immediately 