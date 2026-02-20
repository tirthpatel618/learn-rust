
// structs
struct Vec2 {
    x: f64,
    y: f64,
}


fn main() {
}

fn inits() {
    //order doesn't matter when initializing a struct, but names do
    let v1 = Vec2 { x: 1.0, y: 2.0 };
    let v2 = Vec2 { y: 3.0, x: 2.0 };

    //can also init structs from other structs 
    let v3 = Vec2 { x: 4.0, ..v1 }; // v3.y will be same as v1.y
    //or all the fields
    let v4 = Vec2 { ..v1 }; // v4.x and v4.y will be same as v1.x and v1.y



    println!("v1: ({}, {})", v1.x, v1.y);
    println!("v2: ({}, {})", v2.x, v2.y);
}

//methods

struct Number {
    odd : bool,
    value: i32,
}

impl Number {
    fn is_strictly_positive(self) -> bool {
        self.value > 0
    }
}

fn methods() {
    let n = Number { odd: true, value: 5 };
    println!("Is n strictly positive? {}", n.is_strictly_positive());
}

//mutability
struct Point {
    x: f64,
    y: f64,
}

fn mutability() {
    let mut p = Point { x: 1.0, y: 2.0 };
    p.x = 3.0; // allowed because p is mutable
    println!("Point p: ({}, {})", p.x, p.y);
}