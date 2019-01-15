extern crate crate_lib;

fn main() {
    crate_lib::public_function();

    // Error! `private_function` is private
    //crate_lib::private_function();

    crate_lib::indirect_access();
}


// $ rustc executable.rs --extern rary=library.rlib && ./executable
