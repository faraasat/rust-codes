#![allow(unused_variables)]
#![allow(dead_code)]

use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");  // Result enum will be returned Variant Ok or Err
    let f = match f {  // This will match that if the variable f has Ok Variant or Error Variant
        Ok(file) => file,
        Err(error) => match error.kind() {  // Result::Err(io::Error) and io::Error is a struct and we are using .kind() method on io::Error which returns an enum ErrorKind on which we will use a variant NotFound
            ErrorKind::NotFound => match File::create("hello.txt"){
                Ok(f) => f,
                Err(e) => panic!("Error in creating file"),
            }
            _ => { // _ is used as else case
                panic!("You don't have permission to access file")
            }
        }
    };
}
