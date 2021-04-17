#[allow(dead_code)]
#[allow(unused_variables)]

use std::fs::File;
use std::io::{self,Read};
fn main() {

    fn read_username_from_file() -> Result <String, io::Error> {
        let mut s = String::new();
        let mut f = File::open("hello.txt") ? . read_to_string(&mut s) ?; //? is a shortcut for propagating error
        Ok(s) // We have to explicitly return Ok while Err is returned automatically if present
    }
    println!("{:#?}", read_username_from_file());

}
