#[derive(Debug)]
enum Direction {
    UP,
    LEFT,
    RIGHT,
    DOWN,
}

fn print_direction(direction: Direction) {
    // match direction {
    //     Direction::UP => println!("up"),
    //     Direction::LEFT => println!("left"),
    //     Direction::RIGHT => println!("right"),
    //     Direction::DOWN => println!("down"),
    // }
    // direction is not a printable type and to print it we make it derived attribute
    println!("Direction is {:?}", direction)
}

struct Employee {
    id: i32,
    age: i32,
}

fn print_employee(emp: Employee) {
    println!("id = {:?}", emp.id);
    println!("age = {:?}", emp.age);
}

enum PromoDiscount {
    NewUser,
    Holiday(String),
}

enum Discount {
    Percent(i32),
    Flat(i32),
    Promo(PromoDiscount),
    Custom(String),
}

fn main() {
    let direction = Direction::RIGHT;
    print_direction(direction);

    let emp = Employee { id: 0, age: 20 };
    print_employee(emp);

    let n = 6;
    match n {
        3 => println!("three {}", n),
        other => println!("other {}", other),
    }

    let discount = Discount::Flat(30);

    match discount {
        Discount::Flat(2) => println!("flat discount is 2"),
        Discount::Flat(amount) => println!("flat discount is {}", amount),
    }
}
