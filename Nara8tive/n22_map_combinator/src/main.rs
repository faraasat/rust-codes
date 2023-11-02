fn maybe_num() -> Option<i32> {
    Some(2)
}

fn maybe_string() -> Option<String> {
    Some("string".to_owned())
}

fn main() {
    // to much to do sum
    let res_num = match maybe_num() {
        Some(data) => { println!("data + 1 is: {:?}", data + 1) }
        None => (),
    };

    // here comes map combinator
    let res_num_mc: Option<i32> = maybe_num()
        .map(|num| num + 1)
        .map(|num| num + 1);
    println!("{:#?}", res_num_mc);

    let res_num_mc_2: Option<()> = maybe_num()
        .map(|num| num + 1)
        .map(|num| println!("{:#?}", num));

    let res_string = maybe_string()
        .map(|word| word.len() + 1)
        .map(|num| println!("{:?}", num));
}
