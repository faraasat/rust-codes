#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    // If we want the hole backtracking of error than we set back trace as 1. So, in powershell we use Env:RUST_BACKTRACE=1 and in cmd we use set RUST_BACKTRACE=1
    let v = vec![1, 2, 3];
    v[89];  // Here panic of vector file will run
    // The below panic will not run because the first panic has ran and the program has already been quitted
    panic!("Crash and Burn"); // RUST Will unwind here it is default until we change it into another mode in toml file
}


// unwind  => when the rust will panic
// Rust will clean all variables in functions

//abort  => when the rust will panic
// The program will quit immediately and OS will do cleaning

// if we want to turn panic from its default behavior of unwinding we go to cargo.toml and define [profile.dev] where dev means we are using it for development purpose and if we want for release mode we write [profile.release] and then write panic = 'abort'

// if we want to run program in release mode we will use cargo run --release