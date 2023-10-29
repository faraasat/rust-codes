struct Book {
    pages: i32,
    ratings: i32,
}

fn display_page_count(book: &Book) {
    println!("pages={:?}", book.pages);
}

fn display_page_ratings(book: &mut Book) {
    println!("ratings={:?}", book.ratings);
}

fn main() {
    let book = Book {
        pages: 5,
        ratings: 9,
    };
    // below function has become the owner of book and it will drop the variable from the memory if we donot use &, so we will use & in the parameter
    // references are also immutable by default
    display_page_count(&book);
    display_page_ratings(&mut book);
}
