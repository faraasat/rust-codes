fn add_fn(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    // closures are nameless functions
    let add = |a: i32, b: i32| -> i32 { a + b };
    // closures let us define functions in a much shorter way
    let add_short = |a, b| { a + b };

    let add_fn_result = add_fn(2, 3);
    println!("{:#?}", add_fn_result);

    let add_clos_res = add(2, 3);
    println!("{:#?}", add_clos_res);
    
    let add_short_res = add_short(2, 3);
    println!("{:#?}", add_short_res);
}
