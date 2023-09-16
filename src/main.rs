use std::io;

fn main() {
    println!("Type in Fahrenheit and Celsius.");
    let mut input = String::new();

    io::stdin()
    .read_line(&mut input)
    .expect("Error parsing input");

    let fh: i32 = input.trim().parse()
    .expect("please give me correct string number!");

    let result = (fh - 32) * 5/9;

    println!("In Celsius is: {result}");
}