fn parse_id(input: &str) -> Result<u64, String> {
    match input.parse::<u64>() {
        Ok(id) => Ok(id),
        Err(_) => Err(String::from("ID must be a positive number")),
    }
}

fn main() {
    let input = "42";

    let result = parse_id(input);

    match result {
        Ok(id) => println!("Parsed ID: {}", id),
        Err(message) => println!("Error: {}", message),
    }
}

