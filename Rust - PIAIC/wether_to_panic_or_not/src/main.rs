use std::net::IpAddr;

fn main() {
    // We already know that here alway result enum will be okay because it is a valid Ip address but we are bind to use unwrap() with parse
    let home : IpAddr = "127.0.0.1".parse().unwrap(); // Here parse() will parse the string into IpAddr
    assert_eq!(home.is_ipv4(), true);  // This will check that is ip v4 than return true or false and match with true or false of other side and panics on mismatch
    let home1 : IpAddr = "::1".parse().unwrap();
    assert_eq!(home1.is_ipv6(), true);

    let x : u32 = "67".parse().unwrap();// Here parse() will parse the string into i32
}
