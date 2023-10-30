fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn sub(a: i32, b: i32) -> i32 {
    a - b
}

#[test]
fn test_add() {
    let res = add(2, 3);
    let expected = 5;
    assert_eq!(expected, res, "add does not work")
}

#[test]
fn test_sub() {
    let res = sub(4, 3);
    let expected = 1;
    assert_eq!(expected, res, "sub does not work")
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_add_in_mod() {
        let res = add(2, 3);
        let expected = 5;
        assert_eq!(expected, res, "add does not work")
    }
    
    #[test]
    fn test_sub_in_mod() {
        let res = sub(4, 3);
        let expected = 1;
        assert_eq!(expected, res, "sub does not work")
    }
}

fn main() {
    println!("Hello, world!");
    add(2, 3);
    sub(4, 3);
}
