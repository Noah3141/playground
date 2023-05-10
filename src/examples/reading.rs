/* Reading into a String */

                                        /* 
                                        'Result<(), Box<dyn Error>>' is used when using ? 
                                        instead of match, to propogate the error up the stack 
                                        (means "allow function to return unknown-sized return error")
                                        */

                                        use std::io::Read;

// !) File paths are interpreted FROM ROOT DIRECTORY. So files within our project will be referenced with "./src/..."

pub fn read_to_string(path: &str) -> String { 
    // path formatted as a string slice like "./myfile.txt";

    let content: String = match std::fs::read_to_string(path) { // This can return certain errors.
        Ok(content) => content,
        Err(e) => {
            panic!("\n\nCouldn't read the file into a string! \n{e}\n\n")
        }
    };

    content
} // Mind your UTF-8 Encoding! Make sure your file is encoded in this way.


pub fn read(path: &str) -> String {

    let content: Vec<u8> = match std::fs::read(path) {  // Returns result that when successful contains byte-string of data
        Ok(content) => content,
        Err(e) => {
            panic!("\n\nCouldn't read the file into a string! \n{e}\n\n")
        }
    };

    let content = {
        std::str::from_utf8(&content)
        .unwrap()
        .to_string() // String
    };

    content
}

/* For larger files that can't be read all at once */

pub fn buf_read(path: &str) -> String {


    let file: std::fs::File = {
        std::fs::File::open(&path)
        .unwrap() // Result -> Contents if Ok(T)
    };

    use std::{io, fs}; // Bit of abbreviating the next lines

    let reader: io::BufReader<fs::File> = { // BufReader holds file
        std::io::BufReader::new(file)
    };

    let mut output: String = String::new();
    
    use std::io::BufRead;
    for line in reader.lines() {
        match line {
            Ok(line) => output.push_str(line.as_str()),
            Err(e) => panic!("\n\nCouldn't buf-read this line! \n{e}\n\n")
        }
    }
    output
}

/* Reading single byte-steps */

pub fn read_by_bytestep(path: &str) -> String {

    let file: std::fs::File = {
        std::fs::File::open(&path)
        .unwrap() // Result -> Contents if Ok(T)
    };

    use std::{io, fs}; // Bit of abbreviating the next lines

    let reader: io::BufReader<fs::File> = { // BufReader holds file
        std::io::BufReader::new(file) };

    use std::io::Read;
    let mut content: Vec<u8> = vec![];

    for byte in reader.bytes() {
        match byte {
            Ok(c) => content.push(c),
            Err(e) => panic!("\n\nCouldn't byte-read a line! \n{e}\n\n")
        }
    }
    let content = {
        std::str::from_utf8(&content)
        .unwrap()
        .to_string() // String
    };

    content
}


/* Using fs::File::open */
// Here we start using the ? to say "Let the superordinate function unwrap or not"
pub fn read_file(path: &str) -> std::io::Result<String> {

    let mut file = std::fs::File::open(path)?; 
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;


    Ok((contents))
}



/* Test them out in your main with this function */
pub fn test_reading() {

    use crate::examples::*;
    // Having placed a text file in the root folder, adjacent to src:
    let text = reading::read_to_string("test.txt");
    println!("Read to string:\n{text}\n");
    let text = reading::read("test.txt");
    println!("Read:\n{text}\n");
    let text = reading::buf_read("test.txt");
    println!("Buf-read:\n{text}\n");
    let text = reading::read_by_bytestep("test.txt");
    println!("Byte-step:\n{text}\n");
    let text = reading::read_file("test.txt").expect("Couldn't read that file!");
    println!("Read file:\n{text}\n");

    

}