#![allow(unused)]

mod examples;
use examples::{*};

use lib::*; // lib is imported modular-style in the cargo.toml file, and now we can use any file::function in the lib/src folder


fn main() {

    rocket::tester();

}






// functions::function();

    

    // application();
    // applications::application();
    // applications::applications::application();