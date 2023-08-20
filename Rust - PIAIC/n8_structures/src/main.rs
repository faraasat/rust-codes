#[derive(Debug)]
struct Book{
    name: String,
    author: String,
    price: u16,
    availability: bool,
}

fn main() {

    let book_1 = Book {
        name: String::from("Book A"),
        author: String::from("Author A"),
        price: 500,
        availability: true,
    };
    let book_2 = Book {
        name: String::from("Book B"),
        author: String::from("Author B"),
        price: 600,
        availability: true,
    };
    println!("{:#?}", book_1);
    println!("{:#?}", book_2);

    // Mutable Struct
    let mut book_3 = Book {  //We cannot make mutable only one field we have to make whole instance mutable
        // We can also change the order in initialization
        name: String::from("Book A"),
        price: 1000,
        availability: true,
        author: String::from("Author C"),
    };
    book_3.name = String::from("Book CC");
    println!("{:#?}", book_3);

    // calling book instance from build function
    let book_4 = build(String::from("Book D"), String::from("Author D"));
    println!("{:#?}", book_4);

    // Using another instances value
    let book_5 = Book{
        name: String::from("Book E"),
        author: String::from("Author E"),
        price: book_1.price,
        availability: book_1.availability,
    };
    println!("{:#?}", book_5);

    // If except some values all are same as another instance and this is also called struct update syntax
    let book_6 = Book{
        name: String::from("Book F"),
        author: String::from("Author F"),
        ..book_1
    };
    println!("{:#?}", book_6);
}

// Instance created in another function to be called in main function
fn build(name:String, author:String) -> Book{
    Book{
        // name: name,
        // author: author,
        name,  // If keys are same in struct field as parameters we can also write like this and this is called shorthand
        author,
        price: 1200,
        availability: true,
    }
}