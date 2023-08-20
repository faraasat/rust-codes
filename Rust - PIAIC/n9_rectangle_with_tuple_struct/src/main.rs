
#[derive(Debug)]
struct Rectangle{
    height: u32,
    width: u32,
}

fn main() {

    // The empty {} in println shows that this format is for display and scalar values in primitive data types use this because they are printed in same way

    // In printing an object it is not known that either we want commas or want pretty printing thats why we tell that it will not use Display Trait but use a Debug trait because Display trait by default print scalar value but in Debug trait we till compiler how we want the out to tell the compiler that it is a debug trait we use println with {:#?} or {:?} and also we have to explicitly define #[derive(Debug)] with {:?} we have inline kind (in looking) of output and last field does not have comma (,) and with {:#?} we have object kind (in looking) of output and last field has comma (,)

    // Normal Method
    let height = 20;
    let width = 30;
    println!("Area of rect with normal function is: {}", rect(height, width));

    // With Tuple
    let rec1 = (100,5);
    println!("Area of rect using tuple is: {}", area(rec1));

    // With Struct
    let rec_1 = Rectangle{
        height: 100,
        width: 50,
    };
    println!("Area of rect using struct is: {}", area_1(rec_1));

    // With Struct and reference
    let rec_2 = Rectangle{
        height: 100,
        width: 20,
    };
    println!("Area of rect using struct and reference is: {}", area_2(&rec_2));
    println!("rect_1 is {:#?}", rec_2);
    println!("rect_1 is {:?}", rec_2);
}

fn rect(height: u32, width: u32) -> u32 {
    height * width
}

fn area(dimension: (u32,u32)) -> u32 {
    dimension.0 * dimension.1
}

fn area_1(rec: Rectangle) -> u32 {
    rec.height * rec.width
}

fn area_2(rec: &Rectangle) -> u32 {
    rec.height * rec.width
}