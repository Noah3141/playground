use serde::{Serialize, Deserialize};

#[derive(Serialize,Deserialize)] // This struct is labeled with Serde, which Rocket will need to have its Json tools apply to the struct
pub struct MyStruct {
    pub name: String,
    pub id: u64,
}
                // Note, serde is general, so even rocket's serde has a sub-namespace "json", within which Json appears
#[get("/json_through_rocket")]
pub fn basic_json() -> rocket::serde::json::Json<MyStruct> {

    let my_struct = MyStruct {
        name: "Carl Anderson".to_string(),
        id: 456772,
    };
    

    use rocket::serde::json::Json;
    Json(my_struct)
    // Not:  "Json<my_struct>""
}


/* Recall the two syntaxes for telling reqwest's '.json()' which target you what to parse into 
1) explicit typing the output variable 
2) "Turbo fish syntax"

let res: serde_json::Value = reqwest::get("https://www.boredapi.com/api/activity")
        .await
        .expect("Trying to get a response")
        .json()
        .await
        .expect("making Json");



let res = reqwest::get("https://www.boredapi.com/api/activity")
        .await
        .expect("Trying to get a response")
        .json::<serde_json::Value>()              <- Turbo fish 
        .await
        .expect("making Json");

* let my_struct = rocket::serde::json::from_str(r#"{}"#).unwrap();  //!  <-- No turbo fish will work here

*/


#[get("/json_as_Value")]
pub fn serde_json_value() -> rocket::serde::json::serde_json::Value {
    
    let my_struct = rocket::serde::json::serde_json::json!( {"name": "Dan Johnson", "id": 223200} );

}