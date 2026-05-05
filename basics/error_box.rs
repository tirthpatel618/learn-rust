use std::error::Error;
use std::fs;

fn read_username(path: &str) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?; // the ? operator returns the error immediately if there is one
    let username = contents.trim().to_string();

    Ok(username)
}

fn main() -> Result<(), Box<dyn Error>> { // main retunts a result, if it retuens an error, it will be printed to the console
                    // here the () success value is a unit, nothing meaninful
    let username = read_username("username.txt")?;

    println!("Username: {}", username);

    Ok(())
}