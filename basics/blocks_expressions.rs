/* 
Blocks: 
A pair of brackets declare a block, which has its own scope.
*/

fn main() {
    let x = "out";
    {
        let x = "in";
        println!("Inside block: {}", x);
    }
    println!("Outside block: {}", x);

    // this:
    let x = 42;
    // is equivalent to this:
    let x = { 42 };

    let x = {
        let y = 1; 
        let z = 2;
        y + z // the tail - what the whole block will evaluate to
    };
    println!("Value of x: {}", x);
}

// this is also allowed since it's a block that evaluates to an i32
fn fair_dice_roll1() -> i32 {
    4
}

// eveyrhting is an expression, including conditionals
fn fair_dice_roll2() -> i32 {
    let feeling_lucky = true;
    if feeling_lucky {
        6
    } else {
        4
    }
}

// matches are also expressions
// control flow construct that allows you to compare a value against a series of patterns and execute code based on which pattern matches.
//  similar to switch statements in other languages but more powerful and flexible.

fn fair_dice_roll3() -> i32 {
    let feeling_lucky = true;
    match feeling_lucky {
        true => 6,
        false => 4,
    }
}

