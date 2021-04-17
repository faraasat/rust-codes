// Tuple Struct
#[derive(Debug)]
struct Color(i32,i32,i32);

#[derive(Debug)]
struct Points(u32,u32,u32);

// #[derive(Debug)]
// struct Book{
//     // This will require lifetime which ensures that it does not clean the reference until the Structure in Memory
//     name: &str,
//     author: &str,
//     price: u16,
//     availability: bool,
// }

fn main() {
    let black = Color(6,9,0);
    println!("{:#?}", black);
    another_func(black);
    let axis = Points(1,0,1);
    another_func1(axis);
}

fn another_func(x: Color){
    println!("{:#?}", x);
}

fn another_func1(y: Points){
    println!("{:#?}", y);
}