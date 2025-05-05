#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }
    
    fn square(size: u32) -> Self {
        Self {
            height: size,
            width: size
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: dbg!(2 * 30),
        height: 50
    };
    
    let rect2 = Rectangle {
        width: dbg!(3 * 13),
        height: 40
    };
    
    let rect3 = Rectangle {
        width: 20,
        height: 62
    };
    
    println!(
        "The area of the rectangle rect is {} square pixels", rect.area()
    );
    println!("Rec can hold Rec1? Answer = {}", rect.can_hold(&rect2));
    println!("Rec can hold Rec3? Answer = {}", rect.can_hold(&rect3));
    
    dbg!(rect);
    
    let square = Rectangle::square(40);
    println!("A square: {:?}, and area: {}", square, square.area());
}
