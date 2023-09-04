fn main() {
    let x: i32 = add(a: 1, b: 1);
    let y: i32 = addWithReturn(a: 1, b: 2);

    let number: i32 = 42;
    // println is a macro as it has ! in syntax
    println!("hello");
    println!("{:?}", number);
    println!("{:?} {:?}", number, number);

    // Diverging Function
    // Diverging functions never return to the caller, so they may be used in places where a value of any type is expected. unimplemented!() and panic!() are frequently used diverging functions

    fn never_return_fn1() -> ! {
        panic!()
    }

    fn never_return_fn2() -> ! {
        unimplemented!()
    }
}

fn add(a: i32, b: i32) -> i32 {
    // without return statement ';' is not required
    a + b
}

fn addWithReturn(a: i32, b: i32) -> i32 {
    return a + b;
}
