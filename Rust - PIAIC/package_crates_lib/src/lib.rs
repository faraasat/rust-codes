//Modules, one module can encapsulate other modules in itself and a module can have functions, struct and enums
#![allow(dead_code)]  // It is used to tell that do not show warning if function is not used
#![allow(unused_variables)]
mod customer_experience {

    // front_house is the parent of hosting and server
    pub mod front_house{

        // Here hosting and server are sibling modules
        pub mod hosting {
            pub fn add_to_wait_list() {

            }
        }

        mod server {
            fn take_order () {

            }
            fn order_server () {

            }
            fn payment () {

            }
        }

    }

    mod dining {
        pub fn eat_at_restaurant () {
            // Absolute path but we have to change the path if we chang module level and it will fail if we wrap front_house and eat_at_restaurant in customer_experience
            // crate::front_house::hosting::add_to_wait_list();
            crate::customer_experience::front_house::hosting::add_to_wait_list();
            // front house module is at same level as eat_at_restaurant, the relative path fails when the eat_at_restaurant is wrapped inside another module dining because eat_at_restaurant is now not relative to the front house
            // front_house::hosting::add_to_wait_list();
            // We here use super because eat_at_restaurant is not relative to front_house
            super::front_house::hosting::add_to_wait_list();

        }
    }

}

fn serve_order () {

}

mod back_of_house {

    fn deliver () {
        cook_order();
        super::serve_order();
    }

    fn cook_order () {}

}

pub fn details () {
    println!("The dead line for this project is 2025");
}
