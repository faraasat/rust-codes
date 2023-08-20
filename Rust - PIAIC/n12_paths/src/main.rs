// Using public path of another file
use paths::hosting;
use paths::my_main;

fn main() {
    hosting::add_to_wait_list();
    my_main();
}