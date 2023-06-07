#![allow(unused)]
use lib::{*};
#[macro_use] extern crate rocket;

// use lib::*; 
/// Multi
/// line
// Normal annotation 
// // This is no longer relevant 
// !This is bad! 
// todo: Remember this
// *This is another tone asfasdf 
/*  This is a long block comment */
// ? This asks a question

/* 
!lib is imported modular-style in the cargo.toml file, and now we can use any file::function in the lib/src folder, such as "rocket::launch()" ,
if that file is adjacent to lib.rs, and imported inside lib with "pub mod rocket"
In the current structure, rocket is actually a folder-module, containing its hub "mod.rs", and other files. 
todo: This means we need lib::rocket::task::tester()
Use lib::*; works on lib.rs, not the folder, so there is no lib::lib. You're importing lib.rs, any module in it, allowing for rocket::task::ignite();
*Cargo.toml marks the beginning of any package.
*/ 

mod examples;
use examples::{*};

// !) Remember to await async functions here to!

#[rocket::main]
async fn main() {

    hash_maps::modifying::modify_in_place();

}

