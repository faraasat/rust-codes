// 'a is the lifetime annotation, lifetime annotation does not change the lifetime it just creates the relation
// it means we are returning x and y so ' will make the return lifetime which ever is the shortest b/w the x and y
fn shortest<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
    if *x < *y { x } else { y }
}

struct Test<'a> {
    a: &'a str,
}

impl<'a> Test<'a> {
    fn ret_a(&self, b: &str) -> &str {
        self.a
    }
}

fn main() {
    // lifetime is the lifetime of a reference

    let a: i32 = 5;

    {
        let b = a;
        // a=&b;
        // after this a will become dangling because b is assigned in inner scope
    }

    let x = 3;
    let y = 2;
    let res = shortest(&x, &y);

    println!("{:#?}", res);

    let x2 = 3;
    let res2;
    {
        let y2 = 2;
        res2 = shortest(&x2, &y2);
        println!("{:#?}", res2);
    }
    // will give error b/c the lifetime of res2 is ended above
    // println!("{:#?}", res2);

    let test = Test {
        a: "some_string",
    };
    let res_3 = test.ret_a("asdas");
    println!("{:#?}", res_3);
}
