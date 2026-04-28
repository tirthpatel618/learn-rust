

fn main() {
    let name = String::from("Tirth");
    let other = name; // MOVE

    println!("{}", other); // this will work because other is now the owner of the string
    // string owns heap memory, rust moves ownership
    // println("{}", name) // this will not work because name is no longer the owner of the string
    print_name(&other); // we can pass a reference to other, which allows us to use the string without taking ownership
}

fn print_name(name: &str) { // use str when you only need to read, use String when you need to modify
    println!("Name: {}", name);
}