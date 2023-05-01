### Crate docs: https://docs.rs/reqwest/0.11.17/reqwest/

---

# Async vs Blocking

## Async

    The reqwest::Client is asynchronous.

    To make the main.rs asynchronous, you'll need a dependency that offers an async runtime. Tokio is by far the most common.

_Cargo.toml_: <br/>
`[dependencies]` <br/>
`reqwest = { version = "0.11", features = ["json"] }` <br/>
`tokio = { version = "1", features = ["full"] }`

    Tokio then offers a macro decorator that can be added above the main function, which will be unfolded at compile time into code that causes our application to run asynchronously, and be able to utilize awaiting.

`#[tokio::main]` <br/>
`async fn main() -> ...`

## Blocking

    The reqwest::blocking API contains versions for a blocking runtime.
