fn main() {
    // String literal &str and strings are different str has fixed size white string has dynamic size

    let mut a = String::from("Hello World"); // This is dynamic string and saved in heap
    // :: is an operator used for name spacing while from is a function
    a.push_str(" people");
    println!("{}", a);

    let m = "Hello World";
    // m.push_str(" People");  We cannot do this because string literal is always saved in stack
    println!("{}", m);

    // Data on stack can be copied but data on heap cannot be copied, data on heap is moved

    let a = 5;
    let b = a;
    println!("{}, {}", a,b);

    let s1 = String::from("Hello");
    let s2 = s1;  // The value of s1 is moved to s2 to save variables from double free error and also in rust there is only one owner therefore the variable s1 will be un-validated and no longer contain the pointer to heap
    println!("{}",s2);

    // We cannot do shallow copy in rust but deep copy is possible but it is not preferable
    // Shallow copy means more than one variable is pointing to the same memory location
    // Deep copy means both variable copy both heap and stack data this is because both variables will point to different locations and both variables will become separate entities`. Also in rust copy trait is not applied by default.
    let ss1 = String::from("Hello World");
    let ss2 = ss1.clone();  // This is deep copy
    println!("ss1: {} \nss2: {}",ss1,ss2);

    let s = String::from("Hello There!");
    take_ownership(s);  // s is now moved in this function because its datatype is not primitive so it cannot be used here again
    // println!("{}",s);  We cannot do this

    let x = 5;
    makes_copy(x);  // This is primitive datatype therefore it will be copied and can also be used here
    println!("In main: {}", x);

    let result = gives_ownership();
    println!("gives_ownership: {}", result);

    let st1 = String::from("Pak");
    let result1 = takes_and_give_back(st1);
    println!("takes_and_give_back: {}", result1);

    let sss = String::from("Pakistan");
    let results = length(sss);
    println!("length of word is: {} ", results);

    let sss2 = String::from("Pakistan");
    let (results2, results3) = lengths(sss2);
    println!("The length of word {} is: {}", results3, results2);
}

fn take_ownership(s: String) {
    println!("{}", s);
}

fn makes_copy(x : i32) {
    println!("In Function: {}", x);
}

// When we return from a function the ownership is also returned
fn gives_ownership() -> String {
    let s = String::from("Hello World");
    s
}

fn takes_and_give_back(x : String) -> String{
    x
}

// usize returns value according to the architecture of computer
fn length(name: String) -> usize {
    name.len()
}

// Returning multiple values
fn lengths(name: String) -> (usize, String) {
    (name.len(), name)
}