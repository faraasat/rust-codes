fn main() {
    let mut counter = 5;
    while counter >= 1 {
        println!("counter is {}", counter);
        counter - counter - 1;
    }
    println!("while done");

    let mut i = 5;
    loop {
        println!("i is {}", i);
        if (i <= 4) {
            break;
        }
        i = i + 1;
    }
    println!("loop done");
}
