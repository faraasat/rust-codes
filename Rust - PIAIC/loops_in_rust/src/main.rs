fn main() {

    // loop
    let mut counter = 0;
    let val_counter = loop {  // If we do not write controls on loop it will run infinitely
        println!("Hello world");
        counter = counter + 1;
        if counter == 3 {
            break counter
        }
    };
    println!("{}", val_counter);

    // while loops
    counter = 0;
    while counter < 3 {
        println!("Hello world");
        counter = counter + 1;
    }

    let mut counter_1 = 0;
    let lottery_number = [1,2,45,65,33,77,54,32,44,43];
    while counter_1 < lottery_number.len() {
        println!("{}",lottery_number[counter_1]);
        counter_1 = counter_1 + 1;
    }

    // For loops
    for a in 0..5 {
        println!("{}. Hello world", a);
    }

    // To loop in reverse
    for a in (0..5).rev() {
        println!("{}. Hello world", a);
    }

    // for loop with iter()
    for num in lottery_number.iter() {
        println!("{}", num);
    }
}
