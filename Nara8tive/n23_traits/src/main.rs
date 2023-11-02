// traits are similar to interfaces

struct Dog;
struct Cat;

// we cannot change the signature of a trait
trait Animal {
    fn sound(&self);
    fn legs() {
        println!("it has 4 legs")
    }
    fn ear() {
        println!("it has 2 ears")
    }
    fn color(&self) -> String;
}

impl Animal for Dog {
    // we have to always implement the &self functions
    fn sound(&self) {
        println!("it barks")
    }
    fn color(&self) -> String {
        return String::from("yellow");
    }
}

impl Animal for Cat {
    fn sound(&self) {
        println!("it meows")
    }
    fn color(&self) -> String {
        return String::from("white");
    }
}

fn take_cat_dog(ty: impl Animal) {
    ty.sound();
}

fn main() {
    Dog::legs();
    Dog::ear();

    // we cannot use &self methods without creating instance of it
    let dog = Dog{};
    dog.sound();
    let dc = dog.color();
    println!("{:#?}", dc);

    let cat = Cat{};
    take_cat_dog(cat);
}
