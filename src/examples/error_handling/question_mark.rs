/* First, let's make a function that is choosing not to handle errors within, but rather passes its Result up */

pub fn try_to_open() -> Result<std::fs::File, std::io::Error> {

    let f = std::fs::File::open("absent.txt");
    f // Result<File,Error>
}
/* 
With one step that can fail, it's possible to pass that steps success/failure up the call stack.
If we have multiple spots it can fail, we can code in a way that just returns the last error, and dodges running face first into
any of the other errors. This is pretty unteneble in lots of situations, and makes the error that bubbles up an error that
might not have been from the origin of the issue (but rather an error caused by the last of a series of things that weren't
working)!

So, we need a way to let Rust know that any of a number of different Errors might need to be bubbled up to the caller. If we
do this, the errors that bubble up could be of all sorts of different types and sizes! That means we can't label in the function
signature the exact error type, or even the error's size.

We'll need to say "You could get a success, or get an error, which implements the trait "Error" (it's any sort of Error),
and has an unknown size at compile time!". This allows us to use a short syntax during the lines of the function, the question mark,
which says "Bubble this error variant up to the caller, or this one, or this one, or this one if you error here."

If all the error Results that your function could create are of type std::io::Error, you can simply add question marks to resolve
Results locally. If they are of different types, we'll need the function signature to get ready for anything error-y.

*/

pub fn try_open_alt() -> Result<std::fs::File, std::io::Error> {

    let f = std::fs::File::open("absent.txt")?;
    // f: File
    Ok(f) // Result<File,Error>

} 
//*? Here we redundantly unwrap-or-pass with the question mark, getting at the File, and then wrap it back up during return, to satisfy the signature. 
