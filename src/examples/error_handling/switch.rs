use core::panic;

//? The Standard Result from core::result::Result
enum Result<T, E> {
    Ok(T),
    Err(E),
}


/* Using a switch statement to handle a Result */


pub fn try_open() {

    let f = std::fs::File::open("absent_file.txt");

    //? Result<File, Error> needs a match to pull out either File, or Error

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening file: {error:?}") // Because this does no assignment, f can only ever be File -> Type safety check passes
    };
    /* "file" and "error" are arbitrarily named */

}



/* Using multiple switch statements to dynamically handle different error types */

pub fn smart_try_open() {

    use std::fs::File;

    let f = File::open("absent.txt"); // Try open file, get Result<File, Error>

    let f = match f { // We need to determine which variant we got
        Ok(file) => file, // If it was a success, give me the file
        Err(error) => match error.kind() { // If it was error, determine which kind
            std::io::ErrorKind::NotFound => match File::create("present.txt") { //If it was NotFound, determine what happens when we try to create
                Ok(created_file) => created_file, // If it succeeds, pass up the successfully created file
                Err(e) => panic!("File was not found. Attempted to create, encountered error: {e:?}") // If we fail to create, stop program
            },
            other_error => { // If the error determined from trying to Open the file (line 37) is other any other kind, stop program
                panic!("Error other than NotFound was encountered, no prepared way of handling.\n Error: {other_error:?}")
            } // other_error stands in here as a placeholder of any name, but the catchall is usually "_"
        }
    };
}