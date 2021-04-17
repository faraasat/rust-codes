#![allow(dead_code)]
#![allow(unused_variables)]

use creating_custom_validation::cal;
use std::io;
use std::cmp::Ordering;
use rand::Rng;

pub struct Guess {
    value : i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 101 {
            panic!("You entered the number out of range, you entered {}", value);
        }
        Guess {
            value
        }
    }

    pub fn value (&self) -> i32 {
        self.value
    }
}

fn main() {
    cal::its_positive(8);  // Rust compiler automatically performs type check and panics

    let secret_number : i32 = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is {}", secret_number);

    loop{

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read a line");

        let guess : i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let num = Guess::new(guess);
        let guess = num.value();

        match guess.cmp(&secret_number) {  // cmp returns Ordering enum
            Ordering::Equal => {
                println!("You win");
                break;
            },

            Ordering::Greater => println!("You guess too high"),

            Ordering::Less => println!("You guess too low"),
        };

    }

}
