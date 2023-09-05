fn main() {
    let my_num = 3;

    let is_it_5 = if my_num < 5 { true } else { false };

    let is_it_5 = my_num < 5;

    let trying_to_access = Access::Guest;
    let can_access = match trying_to_access {
        Access::Admin => true,
        _ => true,
    };

    println!("can access: {}", can_access);
}

enum Access {
    Admin,
    Manager,
    Staff,
    Guest,
}
