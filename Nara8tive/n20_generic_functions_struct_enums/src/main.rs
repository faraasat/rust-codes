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

#[derive(Debug)]
struct SingleGen<T>(T);

#[derive(Debug)]
struct Tester<T, Q> {
    score: T,
    id: T,
    address: Q,
    name: Q,
}

#[derive(Debug)]
enum Test2<T, Q> {
    Var1(T),
    Var2(T, Q),
    Var3(T),
}

impl<T: Debug, Q: Debug> Test2<T, Q> {
    fn print_test_res(&self) {
        println!("{:#?}", self)
    }
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

    let sg = SingleGen(2);
    println!("{:#?}", sg);
    let sg1 = SingleGen(3.1);
    println!("{:#?}", sg1);
    let sg2 = SingleGen("string".to_string());
    println!("{:#?}", sg2);

    let test1 = Tester {
        score: 2,
        id: 1,
        address: "abc 123".to_string(),
        name: "Ali".to_string(),
    };
    println!("{:#?}", test1);

    let test2 = Test2::<i32, String>::Var1(2);
    println!("{:#?}", test2);
    let test3 = Test2::<bool, String>::Var2(true, "string".to_string());
    println!("{:#?}", test3);
}
