#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 15,
        height: 30,
    };

    println!("The area is {}", rect1.area());
    println!("The rectangle is {:?}", rect1);
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
}
