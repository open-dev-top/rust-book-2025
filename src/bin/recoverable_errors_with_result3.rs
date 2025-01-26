#![allow(unused)]
fn main() {
    use std::fs::File;
    use std::io::{self, Read};

    fn read_username_from_file() -> Result<String, io::Error> {
        // let username_file_result = File::open("hello.txt");

        // let mut username_file = match username_file_result {
        //     Ok(file) => file,
        //     Err(e) => return Err(e),
        // };
        //// let mut username_file = File::open("hello.txt")?;

        let mut username = String::new();

        // match username_file.read_to_string(&mut username) {
        //     Ok(_) => Ok(username),
        //     Err(e) => Err(e),
        // }
        ////username_file.read_to_string(&mut username)?;
        File::open("hello.txt")?.read_to_string(&mut username)?;
        Ok(username)


        // fs::read_to_string("hello.txt")
    }
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
