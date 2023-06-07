use std::collections::HashMap;


pub fn basic_access() -> () {

    let mut dictionary: HashMap<String, u32> = HashMap::new();
    println!("{}", dictionary.capacity());

    println!("Initialized dictionary as: {:?}", dictionary);

    dictionary.insert("Key_1".to_string(), 39);
    println!("{}", dictionary.capacity());

    let this_value = dictionary["Key_1"]; // Will panic if no such key
    println!("{}", this_value);

    let some_value: Option<&u32> = dictionary.get("Key_1"); // Input can be more loosely like the real key because of signature

    let a_value: Option<&mut u32> = dictionary.get_mut("Key_1");

    // None of these access techniques allow changing the actual entry in the hashmap. We can just read out from it. See modifying.rs
}

pub fn unsure_access() -> () {
    // ?) What if we are accdentally acting on a nonexistent key?

    let mut dictionary: HashMap<String, u32> = HashMap::new();
    let value_at_key = dictionary.entry("Key_47".to_string())
        .or_insert(2);



}