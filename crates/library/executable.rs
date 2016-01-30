//executable.rs
//Link to Library, import items under the rary module
//
extern crate rary;

fn main() {
    rary::public_function();

    //Error! private_funtion is private!
    //rary::private_function();

    rary::indirect_access();
}

