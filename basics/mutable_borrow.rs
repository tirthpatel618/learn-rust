fn main() {
    let mut title = String::from("learn rust"); // variable must be mutable
    
    add_prefix(&mut title);  //pass a mutable borrow   
    println!("{}", title); 

    /* 
    let a = &mut title;
    let b = &mut title; // this will not work because we cannot have two mutable borrows of the same data at the same time
    */
}

fn add_prefix(title: &mut String) {
    title.insert_str(0, "[TODO] ");
}