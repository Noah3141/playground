use serde::{Serialize, Deserialize}; // Bring these into scope

#[derive(Serialize, Deserialize)] // So that we can write it like this, which adds markers which will unfold into code at compile time, which sets up serde for these data structures
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

/* Turning JSON into a struct variable */

// ?) In order to convert into a Rust variable, we need a variable defined that is able to catch that JSON.
// ?) In order to convert into a Rust variable, we need a variable defined that is able to catch that JSON.
// !) let stru = serde_json::from_str( &json_string );
// ?) This code errors, because 'serde_json::from_str' needs to know what kind of structured variable to parse it into.
// ?) Your first option is to use a converter online, to take your example JSON (say, from an API, where you can't control what the JSON format is,
// ?) and need to build a struct to catch it), and generate a valid struct to match.
// ?) But what happens when we try to put JSON into a struct that is smaller than the JSON?

/** 
Put this in main.rs

json::json_to_struct::print_struct_from_larger_json(); // Runs
json::json_to_struct::print_struct_from_matching_json(); // Runs
json::json_to_struct::print_struct_from_mismatching_json(); // Panics
json::json_to_struct::print_struct_from_smaller_json(); // Panics

 */

pub fn print_struct_from_larger_json() -> () {
    let j = serde_json::json!({ // ?) Notice, of course, this block, like any incoming JSON, has no struct name/label, hence the need to specify target struct below
        "name": "Whiskers", 
        "age": 6, 
        "breed":"Ragdoll",
        "friendly": Friendliness::Mean, 
        "friends": vec![                // !) This field wasn't defined in our Cat!
            Cat{
                name:"Sam".to_string(),
                age: 12,
                breed:"Siamese".to_string(),
                friendly: Friendliness::Skittish,
            }
        ]
    });
    let whiskers: Cat = {serde_json::from_value(j) // Try to convert me to Cat from Value 
                                    .unwrap() //? Cat (or Panic) 
    };
    println!("\nWhisker's age is {} (printed from larger JSON)\n", whiskers.age);
    println!("There is no Rust \"friends\" field for our created struct 'whiskers.' Serde_json came across it, had no spot to put it, and tossed it out.\n ");
}

    // ?)) Okay, let's try that again, but this time create a JSON string that matches the pattern we state as a Cat

pub fn print_struct_from_matching_json() -> () {
    let j = serde_json::json!({ // There are other ways to hold JSON, like a string literal, whereupon we'd use "serde_json::from_str" below
        "name": "Whiskers", 
        "age": 6, 
        "breed":"Ragdoll",
        "friendly": Friendliness::Mean, 
        
    });
    let whiskers: Cat = {serde_json::from_value(j)
                                    .unwrap()
    };
    println!("Whisker's age is {} (printed from matching JSON)\n", whiskers.age);
    println!("Whisker's temperament is stored in an enum, which doesn't display."); //? You can't display raw enum value!
    println!("Whisker's breed is {}\n\n", whiskers.breed);
}

    // ?) Next you might ask, what if I make a struct to catch the JSON, but the JSON provides a different type of value than I had expected?
    // ?) Here, a JSON string (Value, technically) is created that offers breed as a Vec, rather than a String.

    // !) Panics
pub fn print_struct_from_mismatching_json() -> () { 
    let j = serde_json::json!({
        "name": "Whiskers", 
        "age": 6, 
        "breed":vec!["Ragdoll","Mancoon","What other cat breeds even are there"],
        "friendly": Friendliness::Mean, 
        
    });
    let whiskers: Cat = {serde_json::from_value(j)
                                    .unwrap() // ! This line will error, because from_value tries to understand the JSON according to struct definition, and panics at the unexpected Vec
    };
    println!("Whisker's age is {} (printed from mismatching JSON)\n", whiskers.age);
    println!("Whisker's breed is {}\n\n", whiskers.breed);
} // !) This means that if the background API changes the JSON structure, your structs will mismatch and error

    //? ) If we try to convert JSON that doesn't have all the field of our struct, into that struct, we will error out.

    // !) Panics
pub fn print_struct_from_smaller_json() -> () {
    let j = serde_json::json!({ // here we make a serde_json::Value
        "name": "Whiskers", 
        "age": 6, 
        // !) No breed or friendliness given!
    });
    let whiskers: Cat = {serde_json::from_value(j) // Try to convert me to Cat from Value 
                                    .unwrap() // !) Panic occurs here, because of missing breed (doesn't even get to the missing 'friendliness')
    };
    println!("\nWhisker's age is {} (printed from larger JSON)\n", whiskers.age); // Try to print a value we did receive
    println!("There is no Rust \"friends\" field for our created struct 'whiskers.' Serde_json came across it, had no spot to put it, and tossed it out.\n ");
}

/* 
Check out Rocket's built-in serde tools, which instead look like rocket::serde::json
 */


/* Figuring out how some JSON should be caught with a struct */

