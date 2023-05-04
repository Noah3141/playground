/* 
The Routes file should contain only the route, handler, and be the level of analysis that would be comprehensible 
to the webpage "user" inititating the request 
*/

use crate::utils::*; // utils is a module found in the first layer, we are currently inside mod routes, so we can use crate::utils to anchor the reference
// Routes can return JSON, text as a String, or HTML pages, and Rocket will ensure that the browser receives a displayable format

#[get("/world")] //? Rocket will allow you to serve up a &str (so long as its lifetime is the length of the program), and the browser will render a basic page
pub fn world() -> &'static str {
    hello::print_hello() 
}

#[get("/data")] //? Here we see that you can create JSON manually within Rust, and send it, but the response type is technically still text
pub fn data() -> &'static str {
    hello::print_data()
}