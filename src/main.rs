#![allow(unused)]

mod examples;
use examples::{*};

use lib::*; 
/* 
lib is imported modular-style in the cargo.toml file, and now we can use any file::function in the lib/src folder, such as "rocket::launch()" ,
if that file is adjacent to lib.rs, and imported inside lib with "pub mod rocket"
In the current structure, rocket is actually a folder-module, containing its hub "mod.rs", and other files. 
This means we need lib::rocket::task::tester()
*/ 


fn main() {

    rocket::task::ignite();

}






// functions::function();

    

    // application();
    // applications::application();
    // applications::applications::application();