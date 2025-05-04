#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn fits_into(&self, other: &Rectangle) -> bool {
        if self.height <= other.height &&
            self.width <= other.width {
            return true;
        }
        
        false
    }
}

fn main() {
    let rect = Rectangle {
        width: dbg!(2 * 30),
        height: 50
    };
    
    let rect2 = Rectangle {
        width: dbg!(3 * 13),
        ..rect
    };
    
    println!(
        "The area of the rectangle rect is {} square pixels", rect.area()
    );
    println!("Rec fits Rec1? Answer = {}", rect.fits_into(&rect2));
    
    dbg!(rect);
}
