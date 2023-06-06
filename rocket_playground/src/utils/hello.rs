
pub fn print_hello() -> &'static str {
    "Hello, world!"
}


pub fn print_data() -> &'static str  {

    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;
    
    data
}