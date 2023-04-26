/* Read a line of CSV in input */

pub fn readline_csv() { //! Not yet sorted out?

    let mut rdr = csv::Reader::from_reader(std::io::stdin());
    for result in rdr.records() {
        let record = result.expect("a CSV record");
        println!("{record:?}");
    }
}

pub fn readfile_csv() -> Result<(), Box<dyn std::error::Error>> {

    fn get_first_arg() -> Result<std::ffi::OsString, Box<dyn std::error::Error>> { //Take a variable kind of error
        match std::env::args_os().nth(1) { // Get me the first argument in the commandline that called our .exe
            None => Err(From::from("Expected 1 arguments, but got none\n")), // If that get returned empty, return error
            Some(arg) => Ok(arg), // If you got something, return an Okay variant of Result, instead of Err
        }
    }

    let file_path = get_first_arg()?; // Get me first argument, if error, pass it up the stack to readfile_csv
    let file = std::fs::File::open(file_path)?; // Try to open the file at that path, else pass error up
    let mut rdr = csv::Reader::from_reader(file); // Create the default csv::Reader and give it the file
    for result in rdr.records() { // for row-read-attempt in the rows
        let record = result?; // call it record if the row-read-attempt succeeds
        println!("{record:?}"); // Print the successful row-read
    }
    Ok(())
} 
//? You will need to run > ./target/debug/playground "./src/examples/csv/example.csv"
// This runs the .exe found in ^            with the argument ^






//*  Excel Files    */ 

/* Get the contents of a specific cell */
pub fn get_cell() -> () {


}

/* Get the contents of a row or column */




/* Import the whole sheet into a datatype */


