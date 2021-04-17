fn main() {

    let x = 3;
    //In match we matches pattern and do not check condition unlike if
    match x {
        // left side is called left arm right side is called right arm and match pattern with the given parameter according to left and right arms
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("None"),
    }

    let my_coin = Coin::Dime;
    let value = value_in_cents(my_coin);
    println!("{:?}", value);

    let my_coins = Coin::Penny;
    let values = value_in_cents(my_coins);
    println!("{:?}", values);

    let my_coin_1 = Coin::Quarter(UsStates::Alabama);
    let value_1 = value_in_cents(my_coin_1);
    println!("{:?}", value_1);

    let four = Some(4);
    let res = plus_one(four);
    println!("{:?}", res);

    let val = plus_one(None);
    println!("{:?}", val);

}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsStates),
}

#[derive(Debug)]
enum UsStates{
    Alabama,
    Alaska,
}

fn value_in_cents(coin: Coin) -> u8{

    match coin {
        Coin::Penny => {
            println!("Lucky! Penny");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // If the any variant has more variants
        Coin::Quarter(state) => {
            println!("{:?}", state);
            25
        }
    }

}

// Using match with Option enum
fn plus_one (x: Option<i32>) -> Option<i32> {

    match x {
        None => None,
        Some(i) => Some(i+1),
    }

}