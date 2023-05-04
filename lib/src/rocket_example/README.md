You'll want to set up Rocket with an async runtime. Rocket, unlike most web frameworks, has a built in recreation of Tokio, allowing you to add
`#[rocket::main]` over your `async fn main()`, instead of the most common `#[tokio::main]`
