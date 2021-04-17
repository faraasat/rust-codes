fn main() {

    // String is also like a vector
    let s = String::new();
    println!("{}", s);

    let s1 = String::from("IoT");
    println!("{:?}", s1);

    // converting str to string
    let s = "Hello world";
    let data = s.to_string();
    println!("{}", data);

    //Conversion by passing string literal
    let datum = "Hello world".to_string();
    println!("{}", datum);

    // Updating a string
    let mut ss = String::from("foo");
    let ss1 = " bar ";
    ss.push_str(ss1);
    println!("{}", ss);

    // Pushing a character in a string
    let ss2 = 'e';
    ss.push(ss2);
    println!("{}", ss);

    // Concatenation with +
    let ss3 = String::from("foo");
    let ss4 = ss3 + ss1;
    println!("{}", ss4);

    // Two concatenate more than two strings we have to use reference instead of first value
    let ss5 = String::from("tic");
    let ss6 = String::from("tack");
    let ss7 = String::from("toe");
    let ss8 = ss5 + "-" + &ss6 + "-" + &ss7;
    println!("{}", ss8);

    // We can also concatenate using format macro
    let ss9 = String::from("tic");
    let st = format!("{}-{}-{}",ss9,ss6,ss7);
    println!("{}", st);

    // Indexing cannot be done in String in heap directly
    // let st1 = String::from("Pakistan"); // This type of indexing is not possible in string because they are stored in heaps
    // println!("{}", st1[0]);

    // English character occupies 1 byte while other languages character occupies 2 bit
    let length1 = String::from("Hello world").len();
    println!("{}", length1);
    let length2 = String::from("Привет мир").len();
    println!("{}", length2);

    // Indexing in str
    let hello = "Hello World";
    let index_data = &hello[0..1]; // Since this is english thats why this code will be write but if it was a language another than english than we have to use [0..2] for first character and so on because they take 2 bytes for one character also in [0..1] 0 is inclusive while 1 is exclusive
    println!("{}", index_data);

    // Iterating on string using chars()
    for c in "PIAIC_IoT".chars() {
        print!("{}\t", c);
    }
    println!("");

    for c in "ہیلو دنیا".chars() {
        print!("{}\t", c);
    }
    println!("");

    // Iterating on string using bytes() which will return utf-8 code
    for c in "PIAIC_IoT".bytes() {
        print!("{}\t", c);
    }
    println!("");

    for c in "ہیلو دنیا".bytes() {
        print!("{}\t", c);
    }
    println!("");



}
