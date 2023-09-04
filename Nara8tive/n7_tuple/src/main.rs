fn main() {
    let tup1 = (1, 5, 7, 40);
    println!("{:?}", tup1.0);
    println!("{:?}", tup1.1);
    println!("{:?}", tup1.2);

    let tup2 = (1, "RUN", true, 3.7);
    println!("{:?}", tup2.0);
    println!("{:?}", tup2.1);
    println!("{:?}", tup2.2);
    println!("{:?}", tup2.3);

    let (x, y, z) = one_two_three();
    println!("{:?}", x);
    println!("{:?}", y);
    println!("{:?}", z);
}

fn one_two_three() -> (i32, i32, i32) {
    (1, 2, 3)
}
