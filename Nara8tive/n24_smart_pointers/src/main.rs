use std::rc::Rc; // rc is a reference counter, how many time the certain data is referenced
use std::cell::RefCell;

#[derive(Debug)]
enum List {
    // wrap List in Box else it will become infinite
    Value(i32, Box<List>),
    Null,
}

fn main() {
    // smart pointers are data structures that act as pointers and have additional metadata

    let a = Box::new(9);
    // this will move a to b
    let _b = a;
    // println!("{:#?}", a);

    let l = List::Value(0, Box::new(List::Null));
    println!("{:#?}", l);

    // string in heap and we wrapped in rc
    let rc_a = Rc::new(String::from("string"));
    println!("number of owners of rc_a {:#?}", Rc::strong_count(&rc_a));
    let rc_a_2 = Rc::clone(&rc_a);
    println!("number of owners of rc_a_2 {:#?}", Rc::strong_count(&rc_a_2));

    let x = RefCell::new(9);
    {
        let mut borrowed = x.borrow_mut();
        *borrowed += 1;
        println!("borrowed value mutable => {:#?}", borrowed);
    }

    let borrowed = x.borrow();
    println!("borrowed value immutable => {:#?}", borrowed);
}
