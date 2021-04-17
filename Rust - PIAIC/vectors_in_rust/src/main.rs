#![allow(dead_code)]
#![allow(unused_variables)]

// Vector is a dynamic array
// Vectors are dropped at the end of the scope and become invalid just like other variables

fn main() {
    // Defining empty vector we have define its datatype
    let mut v : Vec<i32> = Vec::new();
    println!("{:?}", v);

    // If we directly give data then it is not necessary to define datatype and here vec! is a macro
    let mut v1 = vec![12, 34, 64, 78, 90];
    println!("{:?}", v1);

    // pushing value in vector
    v.push(50);
    v.push(39);
    v.push(12);
    println!("{:?}", v);

    // To remove the last index of the vector
    v1.pop();
    println!("{:?}", v1);

    // To access a particular value from vector
    let element1 = v1[2];
    println!("{}", element1);

    // To access a particular value as reference and mentioning datatype is not mandatory
    let element2 : &i32 = &v1[2];
    println!("{}", element2);

    // To access with get and It return Option enum
    let element3 = v1.get(1);
    println!("{:#?}", element3);

    // To extract data coming in Some
    match element3 {
        Some(value) => println!("{}", value),
        None => println!("Nothing")
    }

    let element4 = v1.get(4);
    println!("{:#?}", element4);

    // We cannot do this here, we should avoid this
    // let element5 = &v[0];
    // v.push(250);
    // println!("{}", element5);

    // iterating through a vector
    println!("For with reference:");
    for i in &v {
        print!("{}\t", i);
    }

    println!("\nFor without reference:");
    for i in v {
        print!("{}\t", i);
    }

    println!("\nAdding a value to vector in iteration");
    for i in &mut v1 {
        *i += 50;
    }
    print!("{:?}\t", v1);

    // Storing enum in vector
    let row = vec![SpreadCell::Int(50), SpreadCell::Float(56.5), SpreadCell::Text(String::from("Hello"))];
    println!("\n{:?}", row);

}

#[derive(Debug)]
// using enum with vectors
enum SpreadCell {
    Int(i32),
    Float(f32),
    Text(String)
}