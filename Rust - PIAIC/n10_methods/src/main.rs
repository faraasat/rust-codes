#[derive(Debug)]
struct Rectangle{
    height: u32,
    width: u32,
}

// In defining method first parameter is always self(which can be be defined as self, mut self and &self) and self represents the object on which this method is being called. and the datatype of self is same as the implementation block
impl Rectangle {

    fn area(&self) -> u32 {
        self.height * self.width
    }

    // Method with multiple parameters
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }

    // We can also define function in impl block which are called associated functions and it is used for constructor
    fn square (size: u32) -> Rectangle {
        Rectangle {
            height: size,
            width: size,
        }
    }

}

fn main() {

    let rec_1 = Rectangle{height: 100, width: 50};
    let rec_2 = Rectangle{height: 90, width: 40};
    let rec_3 = Rectangle{height: 70, width: 30};
    let result = rec_1.area();
    let result_1 = rec_2.area();
    let result_2 = rec_3.area();
    println!("The area of rectangle is: {}", result);
    println!("The area of rectangle is: {}", result_1);
    println!("The area of rectangle is: {}", result_2);

    let res_1 = rec_1.can_hold(&rec_2);
    let res_2 = rec_2.can_hold(&rec_3);
    let res_3 = rec_3.can_hold(&rec_2);
    println!("The area of rectangle is: {}", res_1);
    println!("The area of rectangle is: {}", res_2);
    println!("The area of rectangle is: {}", res_3);

    // :: is used for name spacing and associated function it means square lies in Rectangle. In this case if we consider Chrome in C/Folder than chrome name space will be c/Folder::Chrome
    let results = Rectangle::square(8);
    println!("{:#?}", results);
}
