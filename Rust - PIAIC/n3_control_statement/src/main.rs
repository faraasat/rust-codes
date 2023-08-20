fn main() {

    let number = 10;

    if number < 0 {
        println!("The number is negative");
    }
    else if number == 0 {
        println!("The number is zero");
    }
    else{
        println!("The number is positive");
    }

    let even = false;

    let number = {
        if even {
            6
        }
        else {
            7
        }
    };

    println!("{}", number);

}
