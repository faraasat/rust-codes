use std::cmp::PartialOrd;
use std::fmt::Debug;

fn print_type<T: Debug>(a: T) {
    println!("{:#?}", a);
}

// tradebound of PartialOrd and Debuf
fn compare<T: PartialOrd + Debug>(a: T, b: T) -> T {
    if a > b {
        if a == b {
            println!("{:?}", a);
        }
        return a;
    }
    return b;
}

fn mul_gen<A: Debug, B: Debug, C: Debug>(a: A, b: B, c: C) {
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);
}

#[derive(Debug)]
struct Test {
    score: i32,
}

fn main() {
    print_type(1);
    print_type(6.7);
    print_type("string");
    print_type(true);

    let test = Test { score: 90 };
    print_type(test);

    let z = compare(5, 3);
    println!("{:#?}", z);

    mul_gen(2, 3, "das");
}
