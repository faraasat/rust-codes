/**
    // Option type represents a value that may or may not be present
    enum Option<T>{
        Some(T),
        None
    }

    // Result type is used for handling operations that may succeed or fail
    enum Result<T,E>{
        Ok(T),
        Err(E)
    }
*/

struct Survey {
    q1: Option<i32>,
    q2: Option<String>,
    q3: Option<bool>,
}

#[derive(Debug)]
enum MenuChoice {
    MainMenu,
    Start,
    Quit,
}

fn get_choice(choice: &str) -> Result<MenuChoice, String> {
    match choice {
        "mainmenu" => Ok(MenuChoice::MainMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err("wrong choice".to_string()),
    }
}

fn print_choice(menu: &MenuChoice) {
    println!("menu choice is {:?}", menu)
}

fn pick_choice(input: &str) -> Result<(), String> {
    let menu_choice = get_choice(input)?; // Ok or Err
    print_choice(&menu_choice);
    Ok(())
}

fn main() {
    let some_value: Option<i32> = Some(3);
    let none_value: Option<i32> = None;

    let ok_value: Result<i32, String> = Ok(6);
    let err_value: Result<i32, String> = Err("some error occured".to_string());

    let survey = Survey {
        q1: Some(10),
        q2: None,
        q3: Some(true),
    };

    match survey.q1 {
        Some(data) => {
            println!("q1 response is {}", data);
        }
        None => println!("No response for q1"),
    }

    // let menu_choice: Result<MenuChoice, String> = get_choice("start");
    // match menu_choice {
    //     Ok(inner_menu_choice) => { print_choice(&inner_menu_choice) }
    //     Err(e) => { println!("{:?}", e) }
    // }
    let choice = pick_choice("mainmenu");
}
