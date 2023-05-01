# A Repo For Exploring the Basics of Rust

### I've tried to make a repo that:

1.  Gives you an example of a complex crate structure, while explaining how it is linked up.
1.  Provides readable code examples to learn all the basic programming tasks, like reading, writing, http requests, servers, and basic manipulations of text data.
1.  Uses minimal scope adjustments so that you can familiarize yourself with which namespaces contain what.
1.  Is set up for easily trying out the code examples in `main.rs`

### You can find videos related to this repo on my [YouTube channel](https://www.youtube.com/@NoahSteckley/videos)

---

<br/>

![Rust Import Structure 1](https://user-images.githubusercontent.com/66894106/234412414-97a3c1b1-e896-45bc-b55b-7dd4f94756d1.jpg)
![Rust Import Structure 2](https://user-images.githubusercontent.com/66894106/234412425-64a3f5cd-9987-40b9-bf72-3fd42f1e0bc8.jpg)

<br/>

## Questions You May Have

> How do I read novel Rust code?

The way I've written most of this code is not immediately beginner friendly. First you do need to know the basics of Rust's syntax. I've done things like package the right hand side of assignment statements inside curly braces, which can add to the apparent complexity, but only if you aren't used to the syntax. In addition, I've scoped entities as little as possible. This leads to very long verbose chunks of code, in a way that is confusing/overwhelming if you don't know how to read it, but easy and useful if you do know: Strings of namespaces look long, but the entity that matters is the final one. Normally written code would only contain the last one or two namespaces in the chain. Here, they are all left in, to get you familiarized with the crates involved.

> What is the difference between all these `std::fs` and `std::io` options?

`std::io` tackles more complicated data types and possibilities, while `std::fs` is better for basic tasks of reading, writing, and saving to file system.

> Why would I ever want to read something in bytes?

> What is the difference between all these "serde" crate/feature imports?

Serde (**ser**ialize-**de**serialize) is capable of converting all kinds of datatypes into all kinds of serializations, not just JSON. Other ones you may have heard of include URL query strings, in the x-www-form-urlencoded format, or YAML, a straight-forward config language.
The "serde" import refers to the overall crate, at [serde.rs](https://serde.rs). The `features = ["derive"]` part of our Cargo.toml states that we want to include the optional portion of their crate that allows for the shorthand macro label above our structs and enums:

`#[derive(Serialize, Deserialize)]`

Which unfolds at compile time into code outlining the traits and abilities of our struct/enum, without us writing it.

So far, that doesn't mean we're doing anything with JSON, nor that we want to call a JSON serialization function! If we indeed want to do that, there is a different dependency, that will work together with all of the above preparation:

https://docs.rs/serde_json/latest/serde_json/
