use std::io;

pub fn get_user_input() {

    let mut input = String::new();

    io::stdin().read_line(&mut input);

}