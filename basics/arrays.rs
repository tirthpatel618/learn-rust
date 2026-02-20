fn main() {
    let arr : [i32; 5] = [1, 2, 3, 4, 5];
    println!("First element: {}", arr[0]);
    println!("Array length: {}", arr.len());
    for i in arr.iter() {
        println!("Element: {}", i);
    }
    
    let arr2 = [0; 10]; // array of 10 elements, all initialized to 0
    println!("Second array: {:?}", arr2);


}