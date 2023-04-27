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