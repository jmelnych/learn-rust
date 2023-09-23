// Signed and unsigned refer to whether it’s possible for the number to be negative—in other words, 
// whether the number needs to have a sign with it (signed) or whether it will only ever be positive 
// and can therefore be represented without a sign (unsigned). 

fn main() {
    let x: i32 = 10;
    if x == 10 {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }
}