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

    //let structified_json = res.json::<BoredRes>().await.unwrap();
    //println!("\n{:?}", structified_json);
    //println!("Your activity is: {}\n", structified_json.activity);

    println!("{:?}", res.text().await.unwrap());

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