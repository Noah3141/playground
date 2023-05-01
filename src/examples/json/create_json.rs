use serde::{Serialize, Deserialize};


//* First we need to make a struct that can be used for the test. Be sure to import this too!

#[derive(Serialize, Deserialize)]
pub struct Dog {
    pub name: String,
    pub age: u16,
    pub breed: String,
    pub tricks: Vec<String>
}

#[derive(Serialize, Deserialize)]
pub enum Friendliness {
    Friendly,
    Skittish,
    Mean,
}

#[derive(Serialize, Deserialize)]
pub struct Cat {
    pub name: String,
    pub age: u16,
    pub breed: String,
    pub friendly: Friendliness
}

#[derive(Serialize, Deserialize)]
pub enum Adoption {
    Adopted(String),
    Rescued,
}

#[derive(Serialize, Deserialize)]
pub struct Pets {
    pub cats: Vec<Cat>,
    pub dogs: Vec<Dog>,
    pub status: Adoption,
}




/* Create a JSON string */

pub fn create_json_inline() -> () {

    serde_json::json!({"name": "Whiskers", "age": 6, "breed":"Ragdoll","friendly": Friendliness::Mean});


}

