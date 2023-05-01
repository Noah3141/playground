/* Command line text */
use std::io;

pub fn get_user_input() {

    let mut input = String::new();

    io::stdin().read_line(&mut input);

    let input = input.trim(); // The user had to type "Here's what I want to say\n"! They hit enter to send it, so that's what the variable contains, unless you trim.

}


pub fn print_responsive_hello() {

    println!("Would you like me to say hello? (y/n)");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap_or_else(|_| {panic!("I can't understand your accent.")});

    let input = input.trim();

    if input.to_lowercase() == "y" {
        println!("Hi")
    } else if input.to_lowercase() == "n" {
        println!("Suit yourself.")
    } else {
        println!("I'm confused.")
    }
}




/* Command line variables */



//?> You can pass arguments during initiating a .exe by listing them following the call, or
//?> You can use "cargo run -- <arg>". Without the space after the dashes, you pass the arg to cargo run. With the space, you pass it to the program.

pub fn print_flagged_hello() { 

    let arg:Result<std::ffi::OsString, ()> = match std::env::args_os().nth(1) { // Get me the first argument in the commandline that called our .exe
        None => panic!("{}", std::io::Error::new(io::ErrorKind::InvalidInput, "Expected 1 argument")), // If that get returned empty, return error
        Some(arg) => Ok(arg), // If you got something, return an Okay variant of Result, instead of Err
    };

    let arg = match arg {
        Ok(string) => {string},
        Err(e) => {panic!("The input you provided didn't parse: {e:?}")},
    };

    println!("Checking your choice (y/n) for whether to say hello...");

    if arg.to_ascii_lowercase() == "y" {
        println!("Hi")
    } else if arg.to_ascii_lowercase() == "n" {
        println!("Suit yourself.")
    } else {
        println!("I'm confused.")
    }
}