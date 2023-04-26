# Template Patterns of Rust Code You'll See

A function which needs to run a task, can error at a number of points, but we don't want to even try to handle those errors _within_ the function. Maybe, instead, we want the code that calls the function to react to the error. Think of an "open file" function. Do we want the function to do gymnastics to recover from errors? What does it know? How to recover depends a lot on the context of why and how and when it was called. As a result we'll use this pattern to say: Try to do this series of things, if any fails, propogate the error up to the calling function, and it can build a structure to re-calibrate, re-orient, re-call the function in a different way.

```
pub fn naive_function() -> Result<(), Box<dyn std::error::Error>> {

    let object = unreliable_task()?;
    do_this_risky_thing()?;

    let successful_output = try_this()?;

    Ok(())
}
```

Okay, what is this?
It has three parts to note:

1. Output type
2. The question marks
3. The floating Ok(())

These, together, make the function work in the context we outlined above.

`Result<(), Box<dyn std::error::Error>>` says: return me an Ok result, or return me an error of who knows what size (because we don't know which line of the function will generate an error!).

Because of this, the function returns the necessary Ok(()) at the bottom of its code, by default. If the program reaches the end, it hasn't errored out anywhere, and can give the okay as a default.

If it errors at any of the lines above, they are ended with question mark syntax, which means "If this line errors, give the error as the return of the current function."

Importantly, the Result's Ok variant we're working with is empty, because we're working on a function that simply does things inside itself, but doesn't return any value (think "send to database" or "create .txt file"). As a result, we get the common pattern of:<br/>

```
pub fn task() -> Result<(), Box<dyn std::error::Error>> {

    <Attempts>?;

    Ok(())
}
```
