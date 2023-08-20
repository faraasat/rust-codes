#[allow(unused_variables)]

use std::fs::File;

fn main() {
    // In unwrap method we do not pass parameter
    // let f = File::open("hello.txt").unwrap();  // If result type is ok then it return file handle while if it shows error type than it panic
    // Expect give us meaningful error
    let f = File::open("hello.txt").expect("This file is not available");
    println!("{:?}", f);
}
