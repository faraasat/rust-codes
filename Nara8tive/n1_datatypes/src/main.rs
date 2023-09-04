/*
    Datatypes

    * Boolean
    * Integer
    * Double amd Float
    * Character
    * String
*/

fn main() {
    let two: i32 = 2;

    let hello: &str = "hello";

    let j: char = 'j';

    let my_half: f64 = 0.5;

    let mut my_name: &str = "Bill";

    let quit_program: bool = false;

    let your_half: f64 = my_half;

    rust_variables();
}

fn rust_variables() {
    // Rust is pretty flexible with kinds of variables, we can define the properties of a variable using keywords, whether it is mutable, immutable const, or static.
    let x = 9;
    let mut z = 5;

    // Sometimes a particular variable is used many times throughout the program, in that case, the feasible thing to do is to declare it as a const, it can not be mutable.
    const MAX_POINTS: u32 = 100_000;

    // Well, static is similar to the const except for the fact that it stays in the program for a long time (it has a static lifetime ) and it can be mutable as well.
    static MAJOR_VERSION: u32 = 1;
    static mut COUNTER: u32 = 0;

    // ● Unsigned integers :- u8, u16, u32, u64, u128
    // ● Signed Integers :- i8, i16, i32, i64, i128
    // ● Floating point :- f32, f64
    // ● Machine Specific integers:- usize/ isize ( Unsigned integer. Same number of bits as the platform's pointer type.)
    // ● Unicode scalar value:- char
    // ● String:
    //      ○ str -> must be heap allocated
    //      ○ &str -> Stack allocated
    //      ○ String -> Heap allocated
    // ● String slice:- &s[0...2]
    // ● Owned string:- String (rust handles string specially )
    // ● Unit type: () is a value of the type () and its purpose is to be useless. Everything in Rust is an expression, and expressions that return "nothing" actually return ()
}
