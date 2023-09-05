struct User {
    user_name: String,
    id: i32,
}

fn print_name(user_name: &str) {
    println!("User Name: {}", user_name);
}

fn main() {
    // in struct it require owned string, so to convert borrowed string to owned string
    let mut user_list = vec![
        User {
            user_name: "john".to_string(),
            id: 0,
        },
        User {
            user_name: "jack".to_owned(),
            id: 1,
        },
        User {
            user_name: String::from("mike"),
            id: 2,
        },
    ];

    for user in user_list {
        print_name(user.user_name);
        println!("User Id: {}", user.id);
    }
}
