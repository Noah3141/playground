/* Rocket, alone, is like Python's flask. We are not writing front end code, but we can render html pages */
#[macro_use] extern crate rocket; // Thus unfolds into a bunch of use statements that make the crate more ergonomic

mod utils;
mod routes;
use rocket_dyn_templates::Template;
use routes::{
    basics::*,
    json_responses::*,
    HTML_endpoints::*,
};


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {

    let _rocket = rocket::build()
        .mount("/", routes![
            world,
            data,
            basic_json,
            serde_json_value,
            render_file::index
        ])
        .attach(Template::fairing())
        .launch()
        .await?;

    Ok(())
    
}