fn main() {
    let x: i32;
    x = 5;

    let y: i32 = 10;

    println!("Hello, world!");
    println!("x: {}, y: {}", x, y);
    let sum = add(x, y);
    println!("Sum of x and y: {}", sum);
    tuples();
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn tuples() {
    let nums : (i32, i32) = (5, 10);
    let (a, b) = nums;
    let c = nums.0;
    let d = nums.1;
    println!("a: {}, b: {}", a, b);
}