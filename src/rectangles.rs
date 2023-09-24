#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Associated Functions -> methods
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 20,
        height: 30,
    };

    let rect2 = Rectangle {
        width: 15,
        height: 20,
    };

    let rect3 = Rectangle {
        width: 25,
        height: 40,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("rect1 is {:#?}", rect1);
    println!("rect1 can hold rect2 - {}", rect1.can_hold(&rect2));
    println!("rect1 can hold rect3 - {}", rect1.can_hold(&rect3));

    let sq1 = Rectangle::square(5);
    println!("sq1 is {:#?}", sq1);
}