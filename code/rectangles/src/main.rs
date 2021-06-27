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
        self.width >= other.width && self.height >= other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 50,
        height: 30,
    };
    let rect2 = Rectangle {
        width: 60,
        height: 30,
    };

    println!("area is {}", rect1.area());
    println!("rect1 can_hold rect2? {}", rect1.can_hold(&rect2));
}

