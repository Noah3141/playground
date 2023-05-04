#[macro_use] extern crate rocket; // Thus unfolds into a bunch of use statements that make the crate more ergonomic

mod utils;
mod routes;
use routes::{
    basics::*,
    json_endpoints::*,
    HTML_endpoints::*,
};


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    

    let _rocket = rocket::build()
        .mount("/", routes![world])
        .launch()
        .await?;

    Ok(())
}