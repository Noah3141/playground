pub fn access_json_literally() {

    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [ 
                "+44 1234567", 
                "+44 2345678", 
                "355 242 4055", 
                "984 432 2217", 
                "+1 880 700 4545"
            ] 
        }"#; // Newlines aren't real in JSON, they just help to read, the r#"   "# syntax is used for some literal interpretation

    let res: Result<serde_json::Value, serde_json::Error> = serde_json::from_str(data);

    if res.is_ok() {
        
        let json: serde_json::Value = res.unwrap();
        // json now has lots of methods to call, but what you probably really want is:
        let name: &str = json["name"] // Value
            .as_str() // Option<&str>
            .unwrap();

        println!("His name is: {name}");

        // // Notice here, the JSON object is treated with "serde_json::Value" at many levels
        let phones: &Vec<serde_json::Value> = json["phones"] // I'm saying "Try to access a so-called 'phones' field"
            .as_array() // I believe it will be a field with a value of list
            .unwrap(); // Go ahead and assume I'm right

        let first_number = phones[1]
            .as_str()
            .unwrap();

        println!("The first number is: {first_number}");

        for i in 0..phones.len() {
            let number = phones[i].as_str().unwrap();
            println!("\nPhone {} is: {number}", i+1);
        }

    } else {
        println!("Could not extract serde_json::Value from the str! {:?}", res.err());
    }
}

    


pub fn get_json_value() -> Result<(), Box<dyn std::error::Error>> {

    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [ 
                "+44 1234567", 
                "+44 2345678", 
                "355 242 4055", 
                "984 432 2217", 
                "+1 880 700 4545"
            ] 
        }"#; // Newlines aren't real in JSON, they just help to read, the r#"   "# syntax is used for some literal interpretation

    let res = serde_json::from_str(data);

    if res.is_err() {
        println!("Could not extract serde_json::Value from the str! {:?}", res.err());
    } else {
        let json: serde_json::Value = res.unwrap();
        let phone_3 = json["phones"] 
            .get("phones") // Get access elements of list by u64, or values of key-value pairs by "key"
            .ok_or("Phones returned None")?
            .get(3)
            .ok_or("index 3 returned None")?; 
        println!("The 4th phone is {}", phone_3.as_str().expect("to be able to make a string!"));
    };

    Ok(())

}

pub fn file_to_field_value() -> String {

    let json_file = std::fs::read_to_string("./src/examples/json/lots_of_pets.json")
        .expect("to have the file there to read");

    let json_value: serde_json::Value = serde_json::from_str(&json_file) // ! Use explicit type annotation to tell serde_json TO what you want FROM str. Value is your fallback, generally, if no struct available
        .expect("to be able to create json Value from the String");

    let third_trick = json_value["dogs"][0]["tricks"][2]
        .as_str()
        .unwrap();

    println!("Max's third trick is {third_trick}");
    
    third_trick.to_string()

}

pub fn insert_field_value() -> () {

    use serde_json::*;

    let json_file = std::fs::read_to_string("./src/examples/json/lots_of_pets.json")
        .expect("to have the file there to read");

    let mut json_value: serde_json::Value = serde_json::from_str(&json_file) // Use explicit type annotation to tell serde_json TO what you want FROM str. Value is your fallback, generally, if no struct available
        .expect("to be able to create json Value from the String");

    let tricks: &mut Vec<serde_json::Value> = json_value["dogs"][0]["tricks"].as_array_mut().unwrap();
    
    tricks.push(json!("Spin around"));
    

    println!("New JSON:\n{}", serde_json::to_string_pretty(&json_value).unwrap())
    
}


