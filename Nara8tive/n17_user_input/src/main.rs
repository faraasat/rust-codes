use std::io;

fn read_input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    // dont put ; to return the value
    Ok(buffer.trim().to_owned())
}

fn main() {
    let mut counter = 0;
    let mut user_input = vec![];
    while counter <= 2 {
        match read_input() {
            Ok(data) => {
                user_input.push(data);
            }
            Err(e) => println!("{:?}", e),
        }
        counter += 1;
    }

    for i in user_input {
        println!("user input is {:?}", i)
    }
}
