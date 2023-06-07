use std::collections::HashMap;


pub fn extend() -> () {

    let mut dictionary: HashMap<String, String> = HashMap::new();

    dictionary.extend(
        HashMap::from([ ("key".to_string(), "value".to_string()), ]));

    dictionary.extend([ // you can .extend() all sorts of types in this same way!
        ("a".to_string(), "1".to_string()),
        ("b".to_string(), "2".to_string()),
    ]);

}


pub fn insert() -> () { 

    let mut dictionary: HashMap<String, u32> = HashMap::new();
    println!("Initialized dictionary as: {:?}", dictionary);

    dictionary.insert("Key_1".to_string(), 32);

    println!("Dictionary after insert: {:?}", dictionary);
    


}