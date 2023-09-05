fn main() {
    let s1 = "hello"; // &str - referenced string
    let s2 = s1;
    println!("{:?} {:?}", s1, s2);

    let s1_owned = String::from("hello"); // owned string
    let s2_owned = s1_owned;
    println!("{:?} {:?}", s1_owned, s2_owned);
}
