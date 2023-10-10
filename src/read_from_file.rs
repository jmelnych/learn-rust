use std::fs::{File, self};
use std::io::{self, Read};

// error handling

fn read_username_from_file_1() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;

    let mut username = String::new();

    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// even more ergonomic way to write it
fn read_username_from_file_3() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// Reading a file into a string is a fairly common operation, so the standard library provides the convenient fs::read_to_string fn
// that opens the file, creates a new String, reads the contents of the file, puts the contents into that String, and returns it. 
fn read_username_from_file_4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn main() {
    let res = read_username_from_file_1();
    println!("{:?}", res);
}