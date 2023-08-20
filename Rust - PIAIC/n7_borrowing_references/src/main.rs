fn main() {

    let a: u8 = 10;  //a has value 10
    let b = &a;  // b points to a
    let c = &b;  //c points to b
    println!("a:{}, b:{} c:{}", a, b, c);
    println!("The address of a is: {:p}", b);
    println!("The address of b is: {:p}", c);

    let s = String::from("Pakistan");
    let result = length(&s);
    println!("The length of {} is {}", s, result);

    let mut st = String::from("Hello");
    change(&mut st);

    let mut sst = String::from("Hello");
    let asd = &mut sst;
    // let bnm = &mut sst; This is not allowed because they are mutable and due to data race which occurs when two or more pointer access the data at same time or at least one pointer is being used to write data or there is no mechanism being used to synchronize access to data or at least one pointer is mutable
    println!("{}", asd);

    // We can perform above thing by changing scopes
    let mut sstr = String::from("Hello");
    {
        let var1 = &mut sstr;
        var1.push_str(" from var1");
        println!("{}", var1);
    }
    {
        let var2 = &mut sstr;
        var2.push_str(" and var2");
        println!("{}", var2);
    }

    //We also cannot take mutable and immutable references in same scope but can do by changing scopes
    let mut stt = String::from("Hello");
    let var3 = &stt;
    let var4 = &stt;
    let var5 = &stt;
    // let mut var6 = &mut stt; Cannot do this but can do this by changing scope
    println!("{}, {}, {}", var3, var4, var5);
    // Here scope of all the pointers v3 v4 v5 is over because they have printed their data and done reading

    //Changing scope of var6
    let var6 = &mut stt;
    println!("{}", var6);

    let res = dangle();
    println!("{} at {:p}", res, &res);
}

// Reference is passed in function
fn length(x: &String) -> usize {
    x.len()
}

// Function with mutable reference
fn change(x: &mut String) {
    x.push_str(" World");
    println!("{}", x);
}

// Dangling Reference
// fn dangle() -> &String {
//     let s = String::from("Hello");  // scope of s is started
//     &s  // pointer is returning
// } //Since scope of s ended therefore the memory will be cleared

// We can set this by transferring ownership and then covert it into pointer
fn dangle() -> String {
    let s = String::from("Hello");
    s
}