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




/* Create a JSON string inside of Rust */

/* Option 1) Create it as a Value type through serde_json */

pub fn create_json_inline() -> () {

    let j = serde_json::json!({
        "name": "Whiskers", 
        "age": 6, 
        "breed":"Ragdoll",
        "friendly": Friendliness::Mean, 
        "friends": vec![
            Cat{
                name:"Sam".to_string(),
                age: 12,
                breed:"Siamese".to_string(),
                friendly: Friendliness::Skittish,
            }
        ]
    });

    println!("Here is your JSON, sir:\n{}", j.to_string())

}

/* Option 2) Create the JSON string literally */



pub fn create_json_literally() -> String {

    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#; // Newlines aren't real in JSON, they just help to read, the r#"   "# syntax is used for some literal interpretation

    println!("\nHere is your JSON, sir:{}\n", data);

    data.to_owned() // If it returned a reference (&str) we'd need a lifetime annotation, but for now we just convert to owned as String
}