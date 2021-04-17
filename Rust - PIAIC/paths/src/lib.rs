#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;  // Hashmap is a struct in a built-in standard library
use rand::Rng;  // Using external library

// use std::fmt;  //standard library of format
// use std::io;  //standard library of I/O
// if we do like use std::fmt::Result; and use std::io::Result then rust will become ambiguous
// We can also correct this ambiguity by aliasing
// use std::io::Result as IoResult;  // This is aliasing

pub fn my_main() {

    let mut contacts = HashMap::new();
    contacts.insert("1", "abc@gmail.com");
    println!("{:?}", contacts);

    // Rust will know not ambiguous of there Result Return type
    // fn function1() -> fmt::Result {}
    // fn function2() -> io::Result<()> {}
    // fn function2() -> IoResult<()> {}

    let secret_number = rand::thread_rng().gen_range(1, 100);
    println!("{}", secret_number);

}

pub mod front_house{

    pub mod hosting {
        pub fn add_to_wait_list() {

        }
    }

    // If we define struct as public than its inner element will remain private until we explicitly define them as public
    #[derive(Debug)]
    pub struct Breakfast{
        pub toast:String,
        seasonal_fruit:String,
    }

    impl Breakfast {
        // This is basically a constructor which will return the instance
        pub fn new (toast:String) -> Breakfast {
            Breakfast{
                // First toast is a field of struct while second toast is of argument
                // toast:toast,    // Writing this will also be acceptable
                toast,
                seasonal_fruit:String::from("Oranges"),
            }
        }
    }

    // In case of enum if we make enum public then all its variants will be public
    pub enum Appetizer {
        Soup,
        Salad,
    }

}

// use crate::front_house::hosting; absolute path
// use self::front_house::hosting;  // Relative path and we can only use this in current file
pub use self::front_house::hosting; // We can use this path in any file
pub fn eat_at_restaurant() {

    let mut meal = front_house::Breakfast::new(String::from("Wheat"));
    println!("{:#?}", meal);
    meal.toast = String::from("Barley");
    println!("{}", meal.toast);

    let meal1 = front_house::Appetizer::Soup;
    let meal2 = front_house::Appetizer::Salad;

    // If we do not want to right path of a function every time
    hosting::add_to_wait_list();
}