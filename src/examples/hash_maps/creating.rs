

pub fn create_dictionary() -> () {

    use std::collections::HashMap;

    let dictionary: HashMap<String, String> = HashMap::new();

    let dictionary_2 = HashMap::from([ ("key".to_string(), 26), ]);

    let map = HashMap::from([
        ("a", 1),
        ("b", 2),
        ("c", 3),
    ]);
    

}