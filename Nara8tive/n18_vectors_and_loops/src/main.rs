#[derive(Debug)]
struct Test {
    marks: i32,
    id: i32,
}

fn main() {
    let mut scores = vec![
        Test { marks: 10, id: 0 },
        Test { marks: 20, id: 1 },
        Test { marks: 40, id: 2 },
        Test { marks: 60, id: 3 },
    ];

    for i in scores {
        println!("id: {:#?}", i);
        println!("id: {} marks {}", i.id, i.marks);
    }
}
