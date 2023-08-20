// here we are accessing io library from std crate
use std::io;

fn main() {
    let mut s = String::new();  // new() is a associated function and String is a type and this function create new, empty string
    // Here stdin is a function from io module and it returns an instance. And then we are using read_line() on the value returned by stdin function and if the io process failed than it will run expect function
    io::stdin().read_line(&mut s)
     .expect("Failed to read a line");
    println!("{}", s);
    let s_integer : u32 = s.trim().parse().expect("Please type a number");
    println!("{}", s_integer);
}
