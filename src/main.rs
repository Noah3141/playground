#![allow(unused)]

use lib::*; 
/// Multi
/// line
// Normal annotation 
// // This is no longer relevant 
// !This is bad! 
// todo: Remember this
// *This is another tone
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


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let client = reqwest::blocking::Client::new();
    let req = client.get("https://catfact.ninja/fact").build().expect("Couldn't build your request!");

    let res = client.execute(req)?;

    let json = res.text().unwrap();

    let fun = json.


    Ok(())
}





fn cool_idea() {



    let mut data = csv::struct_to_csv::CsvData {
        name: String::from("John"),
        money: 400,
        phone: String::from("370-420-5557")
    };

    
    let now = datetime::get_now();

    if let Err(err) = data.write_struct_csv(format!("./{now}").as_str()) {
        println!("{}", err);
        std::process::exit(1);
    }

}