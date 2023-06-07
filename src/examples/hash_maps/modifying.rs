use std::collections::HashMap;



pub fn change_a_value_for_a_key() -> () {

    let mut dictionary: HashMap<String, u32> = HashMap::new();
    println!("Initialized dictionary as: {:?}", dictionary);
    dictionary.insert("Key_1".to_string(), 39);
    
    // !) dictionary["Key_1"] = 22;  << Not a thing, but its error tells you a lot

    println!("Before adding 2: {:?}", &dictionary);

    let key_1_value: &mut u32 = dictionary.get_mut("Key_1").expect("Presence of value");

    // !) println!("Before adding 2: {:?}", &dictionary); << We cannot even attempt a reference to dictionary while a mutable reference has been handed out
    *key_1_value = *key_1_value + 2;
    println!("After adding 2: {:?}", dictionary); // < Here, the compiler knows that nothing new is happening with key_1_value. It can be dropped, and access re-opened.



    // Or, done another way:
    dictionary.get_mut("Key_1").map(|val| *val + 3 );
    println!("After mapping value to value + 3: {:?}", dictionary);


    // Or, you can also do:

    dictionary.insert("Key_1".to_string(), 999); // like .entry(), .insert() requires a closer match to the real type of Key
    println!("After mapping value to value + 3: {:?}", dictionary);

}



pub fn modify_in_place() -> () {

    let mut dictionary: HashMap<String, u32> = HashMap::new();
    println!("Initialized dictionary as: {:?}", dictionary);

    dictionary.insert("Key_1".to_string(), 39);
    println!("Key for the value {} is: Key_1", dictionary["Key_1"]);

    use std::collections::hash_map::Entry;
    let the_value = dictionary.entry("Key_1".to_string()); // Returns Entry<String, u32>. Input must match type of output key exactly, unlike above

    the_value.and_modify(|v| { *v += 1 });
    println!("New key for the value {} is now: Key_100", dictionary["Key_100"]);


    // ?) What if we are accdentally acting on a nonexistent key?

    let _value_at_key = dictionary
        .entry("Key_47".to_string())
        .and_modify(|v| { *v += 1 })
        .or_insert(1);




    let mut map: HashMap<&str, u32> = HashMap::new();

    map.entry("poneyland").or_insert(3);
    assert_eq!(map["poneyland"], 3);
    
    *map.entry("poneyland").or_insert(10) *= 2;
}


pub fn change_a_key_for_a_value() -> () {

    let mut dictionary: HashMap<String, u32> = HashMap::new();
    println!("Initialized dictionary as: {:?}", dictionary);
    dictionary.insert("Key_1".to_string(), 39);
    
    
}

// `.insert(`, `, `, `)`, `.get_mut(`, `).map(|val| { *val`, `; })`, `let val = `, `.entry(`, `).or_insert(`, `)`