// To create lib.rs with command line use  -->  cargo new --lib lib.rs
// main.rs is executable while lib.rs is not
// To call a folder package there should be at least one crate either main.rs or lib.rs
// A package can have many binary crates but a package cannot have more than one lib file and lib crate is not mandatory
// lib.rs is a crate root for library and main.rs is a crate root for binary
// To make more crates we create a folder bin in src and than can make more crates
// By default all things are private in rust unless we define it as public
// main.rs and lib.rs are called crate root because they create crates on the root tree

use package_crates_lib::details;

fn main() {
    details();
}
