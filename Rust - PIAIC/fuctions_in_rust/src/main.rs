fn main() {
    paper();
    square(2, 5);

    // Statement performs an action but never return a value while expression always return a value
    let y = 10; // Statement
    println!("y is {}", y);

    let number = {
        let o = 19;
        o + 1  // This line is an expression
    };  // This whole is an statement
    println!("number is {}", number);

    // Function with return values
    println!("Square of 9 is: {}", sqrt(9));
    let (value, value_1) = cube(5, 10);
    println!("cube of 5 and 10 is: {} and {}", value, value_1);
}

fn paper(){
    println!("1. Add Milk");
    println!("2. Add Butter");
    println!("3. Add Eggs");
    println!("4. Add Sugar");
    println!("5. Stir It");
    println!("6. Heat on Gentle Flame");
}

fn square(x: u32, y: u32){
    let result_1 = x*x;
    let result_2 = y*y;
    println!("square of {} and {} is: {} and {}", x, y, result_1, result_2)
}

// Function with returning more than one
fn cube(x : u32, y : u32) -> (u32, u32) {
    let result_1 = x*x*x;
    let result_2 = y*y*y;
    (result_1, result_2)
}

// Function with returning one value
fn sqrt(x : u32) -> u32 {
    let result_1 = x*x;
    result_1
}