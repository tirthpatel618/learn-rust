fn main() {
    let x: i32;
    x = 5;

    let y: i32 = 10;

    println!("Hello, world!");
    println!("x: {}, y: {}", x, y);
    let sum = add(x, y);
    println!("Sum of x and y: {}", sum);
    tuples();
    numbers();
    types();
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn add2(a: i32, b: i32) -> i32 {
    return a + b;
}

fn numbers() {
    let x: i32 = 5;
    let y: f64 = 3.14;
    println!("x: {}, y: {}", x, y);
    let quotient = 56.7 / 32.2;
}

fn types() {
    println!("Types:");
    let z: bool = true;
    let c: char = 'A';
    let s: &str = "Hello";   
    println!("z: {}, c: {}, s: {}", z, c, s); 
}

fn tuples() {
    let nums : (i32, i32) = (5, 10);
    let (a, b) = nums;
    let c = nums.0;
    let d = nums.1;
    println!("a: {}, b: {}", a, b);
}
