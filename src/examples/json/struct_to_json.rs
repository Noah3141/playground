
use serde::{Serialize, Deserialize};

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




/* A data structure can be converted to a JSON string by serde_json::to_string  */

#[derive(Serialize, Deserialize)]
struct Address {
    street: String,
    city: String,
}

pub fn print_an_address() -> serde_json::Result<()> {
    // Some data structure.
    let address = Address {
        street: "10 Downing Street".to_owned(),
        city: "London".to_owned(),
    };

    // Serialize it to a JSON string.
    let j: String = serde_json::to_string(&address)?;

    let value: serde_json::Value = serde_json::to_value(&address).expect("to convert");
    let string = value.as_str();
    // Print, write to a file, or send to an HTTP server.
    println!("\nYour JSON, sir:\n{}\n", j);

    Ok(())
}

pub fn print_pets() -> serde_json::Result<()> {

    let oscar: Cat = Cat {name:"Tut".to_string(), age: 13, friendly: Friendliness::Friendly, breed: "Egyptian".to_string() };
    
    println!("\n\nHere's the basic JSON of oscar: \n {}\n", serde_json::to_string(&oscar).unwrap());
    println!("Here's the pretty JSON of oscar: \n {}\n", serde_json::to_string_pretty(&oscar).unwrap());
    println!("Notice, the variable name is nowhere to be seen!");

    let few_pets = Pets {
            cats: vec![Cat{name:"Whiskers".to_string(), age: 4, friendly: Friendliness::Mean, breed: "Ragdoll".to_string() },],
            dogs: vec![],
            status: Adoption::Rescued,
    };

    println!("\nHere's what few pets looks like:\n{}\n", serde_json::to_string(&few_pets).unwrap());
    println!("Notice: Vector/list show their brackets even if empty; structs are converted to JSON objects within the JSON object; and Enums variants without held values are represented as a string of their name.");    
    
    let some_pets = Pets {
            cats: vec![Cat{name:"Tut".to_string(), age: 13, friendly: Friendliness::Friendly, breed: "Egyptian".to_string() }],
            dogs: vec![Dog{name:"Max".to_string(), age: 12, breed: "Huskey".to_string(), tricks: vec![] }],
            status: Adoption::Adopted(("From neighbor".to_string()))
    };

    println!("\nHere's what some pets looks like:\n{}\n", serde_json::to_string(&some_pets).unwrap());
    println!("Notice here, that an enum variant that holds a value forms itself into a JSON object, with variant name (PascalCase) as key, with variant value as its pair.");

    let lots_of_pets = Pets {
            cats: vec![Cat{name:"Whiskers".to_string(), age: 4, friendly: Friendliness::Mean, breed: "Ragdoll".to_string() }, Cat{name:"Tut".to_string(), age: 13, friendly: Friendliness::Friendly, breed: "Egyptian".to_string() }],
            dogs: vec![Dog{name:"Max".to_string(), age: 12, breed: "Huskey".to_string(), tricks: vec!["Play dead".to_string(), "Shake hands".to_string(), "Bring beer".to_string()] }],
            status: Adoption::Adopted(("From neighbor".to_string()))
    };

    println!("\nHere's what lots of pets looks like:\n{}\n", serde_json::to_string(&lots_of_pets).unwrap());
    println!("\nHere's what lots of pets looks like, pretty printed:\n{}\n", serde_json::to_string_pretty(&lots_of_pets).unwrap());


    Ok(())
}

