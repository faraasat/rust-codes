
fn main() {

    let student1 = Student::Online;
    let student2 = Student::Onsite;
    println!("{:?}", student1);
    println!("{:?}", student2);

    let four = IPAddrKind::V4;
    let six = IPAddrKind::V6;
    println!("{:?}", four);
    println!("{:?}", six);

    let ip_address1 = IPAddress{
        kind: IPAddrKind::V4,
        address: String::from("127.0.0.1")
    };
    println!("{:?}", ip_address1);
    println!("{:#?}", ip_address1);

    let ip_address_2 = IPAddrKind1::V4(String::from("127.0.0.1"));
    let ip_address_3 = IPAddrKind1::V6(127,0,0,1);
    println!("{:#?}", ip_address_2);
    println!("{:#?}", ip_address_3);

    let msg1 = Message::Quit;
    let msg2 = Message::Write(String::from("Hello how are you"));
    let msg3 = Message::Move{x: 10, y: -9};
    let msg4 = Message::ChangeColor(10,20,30);
    println!("{:?}", msg1);
    println!("{:?}", msg2);
    println!("{:?}", msg3);
    println!("{:?}", msg4);

    let msgs = Message::Write(String::from("Hi! how are you"));
    msgs.call();

    route(four);
    route(six);

    let some_number = Option::Some(5);
    let some_str = Option::Some(String::from("Hello"));
    println!("{:?}", some_number);
    println!("{:?}", some_str);

    // Since option enum is builtin therefore we can use it without Option
    let some_number_1 = Some(5);
    let some_string = Some(String::from("IoT"));
    println!("{:?}", some_number_1);
    println!("{:?}", some_string);

    // To make none Instance and we can give any datatype in <> and to make a variable None we have to define any datatype
    let some_none_val: Option<i32> = Option::None;
    println!("{:?}", some_none_val);

    // This will give error because compiler do not know how to add i8 and Option<i8>
    // let x: i8 = 8;
    // let y: Option<i8> = Option::Some(10);
    // let sum = x + y;
    // println!("{}", sum);
}

// Data type of both online and onsite is same but they are different variants
#[derive(Debug)]
enum Student{
    Online,
    Onsite,
}

#[derive(Debug)]
enum IPAddrKind{
    V4,
    V6
}

#[derive(Debug)]
struct IPAddress{
    kind: IPAddrKind,
    address: String,
}

// Putting data directly into enum without using struct
#[derive(Debug)]
enum IPAddrKind1{
    V4(String),
    V6(u32,u32,u32,u32),
}

// Enum Messages
#[derive(Debug)]
enum Message{
    Quit,  // Unit Struct
    Write(String),
    Move{x: i32, y: i32},  // Tuple Struct
    ChangeColor(u32,u32,u32),  // Tuple Struct
}

// Enum with impl
impl Message{

    fn call(&self){
        println!("{:?}", self);
    }

}

// Passing enum in a function
fn route(x: IPAddrKind){
    println!("{:#?}", x);
}

// It is Option enum which is used to give any value having any data type using some or can give a none value by calling None. We have defined option enum here for our concept and this option enum is builtin in Rust therefore we can directly use it
#[derive(Debug)]
enum Option <T>{
    Some(T),
    None
}