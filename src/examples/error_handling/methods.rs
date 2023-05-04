/* Using unwrap_or_else to provide (closure-based) error handling */

pub fn try_open_else() {

    use std::fs::File;
    use std::io::ErrorKind;

    let f = File::open("absent.txt").unwrap_or_else(|err| {
        if err.kind() == ErrorKind::NotFound {
            File::create("present.txt").unwrap_or_else(|err| {
                panic!("Couldn't open file. Tried to create file, encountered error: {err:?}")
            })
        } else {
            panic!("Problem opening the file: {err:?}")
        }
    });

}


pub fn try_do_or_continue() { // For when you want to run a line which returns nothing, and also catch its error possibility, use:

    fn risky_fn(msg: &str) -> Result<(), std::io::Error> {
        if msg.len() % 4 == 0 {
            Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "You can't input a message with a length divisible by 4."))
        } else {
            println!("Successful print: {msg}");
            Ok(())
        }
    }

    let msg = "Message of a certain length";
    
    match risky_fn(msg) {
        Ok(()) => (),                           //?) Run the line if successful... That's it... 
        Err(e) => println!("\n\n                //?) But if it returns error also print me a bit of info
        There was a problem trying to run risky_fn: \n
        {e}\n\n
        The inputted message was: {msg}\n
        ")
    }
}