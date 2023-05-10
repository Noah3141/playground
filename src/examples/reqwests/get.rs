/* Making GET Requests */ //! Be sure to set up async; see README


    /**
     *  (All these functions have prints built in)
     *  Here, like in reading functions, we are defining a function which either returns a String, or a variable/unknown ("box") type ("dyn") of error.
     *  Thus, the function must end with an Ok() variant of Result, and will hold and pass along a String.
     */

pub async fn get_req_shortcut() -> Result<String, Box<dyn std::error::Error>> {

    let body = reqwest::get("https://www.rust-lang.org") // Returns a Future, which needs to be "awaited"
        .await? // Once awaited, we receive a Response
        .text() // We call a method on the Response struct, receiving a Future, which must be awaited again
        .await?; // Both these await methods are returning Result<T, E>, and are unwrapped inline with the '?', which would cause any errors to immediately return from this function (to 'Box<dyn Error>')

    println!("body = {}", body);

    Ok(body)
} //? Here, we GET request a webpage, and so the response is the HTML for the page.

    /* For blocking:

    let resp = reqwest::blocking::get("https://httpbin.org/ip")?...
    * Notice that the ? mark operator occurs directly after the GET now, and no await(s) occur
    */


    //? The fullscale method makes use of pooled connections, and is better for many requests
    //* The Client has various configuration values to tweak, but the defaults are set to what is usually the most commonly desired value. 
    //* To configure a Client, use Client::builder() [ same thing as ClientBuilder::new() ]. Otherwise Client::new() will provide the defaults.

pub async fn get_req() -> Result<(), Box<dyn std::error::Error>> {

    let client: reqwest::Client = reqwest::Client::new();

    let req = {
                                client // Our reqwest::Client
                                    .get("https://www.boredapi.com/api/activity") // Makes a RequestBuilder (request being built)
                                    .build() // Stamp complete on this request, giving a Result
                                    .unwrap() // Blithely assume Result<Ok(x)>, and give me the contents x
    };

    let res = {
        client.execute(req) // returns a Future
            .await // Takes a future and returns its contains when the operation completes, freeing thread while waiting
            ? // Give me the result Ok contents, else if Err, pass up to function caller
    };

    let structified_json = res.json::<BoredRes>().await.unwrap(); // Can also be written with type specification on the left side of =
    println!("\n{:?}", structified_json);
    println!("Your activity is: {}\n", structified_json.activity);

    //println!("{:?}", res.json::<serde_json::Value>().await.expect("msg"));

    Ok(())

}


#[derive(Debug, serde::Deserialize)] // The following struct will implement Debug and Deserialize traits
pub struct BoredRes { // A struct to capture the JSON response from boredapi.com
    pub activity: String,
    pub r#type: String,
    pub participants: u8,
    pub price: f32, //? We can tell this needs to be a float by the provided .0 in the JSON
    pub link: Option<String>, //? if the API sends an empty string, this will ALWAYS be "Some". Option::None only corresponds to JSON 'null' 
    pub key: String,
    pub accessibility: f32, 
}

pub async fn get_to_text_for_serde_json() {

    let res = reqwest::get("https://www.boredapi.com/api/activity")
        .await
        .expect("Trying to get a response")
        .text()
        .await
        .expect("Trying to put into text");

    let res_json = serde_json::to_value(&res).expect("Trying to make a serde_json::Value out of respose");

    /* Printing out whole json */
    println!("\n Here is the original response text to string pretty: {}", serde_json::to_string_pretty(&res).expect("to string pretty"));
    
    println!("\n Here is the Value printed: {}", &res_json);
    
    println!("\n Here is the Value to string pretty: {}", serde_json::to_string_pretty(&res_json).expect("to string pretty"));

    println!("\n As you can see, if you want clean JSON, don't use .text on the response!");

    /* Getting one field's value */ // !) All of this will work horridly because we 'imported' with .text(), which is riddled with escapes!
    println!("\n Let's print out the activity value:");

    let activity = &res_json["activity"];
    // !) let activity_str =  activity.as_str().expect("Trying to str from Value[index]");

    println!("\n Straight indexing into Value: {}",activity );
    //println!("\n To str: {}", activity_str);


}


pub async fn get_to_json_for_serde_json() {

    let res: serde_json::Value = reqwest::get("https://www.boredapi.com/api/activity")
        .await
        .expect("Trying to get a response")
        .json()
        .await
        .expect("making Json");

        // ?) Here we are passing from reqwest's json function, which can only output a specified type 
        // ?) (sometimes you may want to go straight into a serde labeled struct), to serde_json's Value,
        // ?) Serde_json's Value struct can be more useful, and allow more careful steps. For instance,
        // ?) A json string can either deserialize into your struct, or not. Wholesale. But if you turn
        // ?) it into a serde_json::Value, you can match on each step, and try multiple ways of access a
        // ?) variety of field names and values. This is great when you don't totally know what the JSON
        // ?) response will look like.

        /* //?) This also works the same
        let res = reqwest::get("https://www.boredapi.com/api/activity")
        .await
        .expect("Trying to get a response")
        .json::<serde_json::Value>()
        .await
        .expect("making Json");
        */

    /* Printing out whole json */
    println!("\nHere is the Value printed: \n{}", &res);
    
    println!("\nHere is the original response (now a Value) text to string pretty: \n{}", serde_json::to_string_pretty(&res).expect("to string pretty"));
    
    println!("\nAs you can see, if you want clean JSON, don't use .text on the response!");

    /* Getting one field's value */ 
    println!("\nLet's print out the activity value:");

    let activity = &res["activity"];
    let activity_str =  activity.as_str().expect("Trying to str from Value[index]");

    println!("\nPrinting the straight index into Value: \n{}", activity);
    println!("\nNote the quotation marks! Value can be displayed, but it isn't precisely a string yet.");
    println!("\nTo str: \n{}\n", activity_str);


}