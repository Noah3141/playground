/* Includes creating a file */




/* Create a directory */
pub fn create_dir(path: &str) -> std::io::Result<()> { // create_dir_all will also make chains of directories where parent folders don't yet exist

    match std::fs::create_dir(path) {
        Ok(()) => Ok(()), // if ok, it doesn't hold anything, so just give me a null okay
        Err(_) => panic!("Couldn't create that directory!") // catch any error that occurs and stop the program
    }

}

/* Create a file */
pub fn create_file(path_name: &str) -> std::io::Result<()> {

    let file: std::io::Result<()> = match std::fs::File::create(path_name){
        Ok(f) => {
            println!("{f:?}"); // The :? means to print in debug mode, allowing the printing of datatypes that aren't just a form of plain text, like a vector. Data that doesn't 'display' in the normal sense can often be printed with this "print ugly" :?
            Ok(())
        },
        Err(e) => panic!("\n\nCouldn't create the file! \n{e}\n\n")
    };
    
    Ok(())
}


/* Moving a file */


/* Renaming a file */



/* Checking if a directory exists */