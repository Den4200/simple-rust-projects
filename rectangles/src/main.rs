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
        self.width > other.width && self.height > other.height
    }
    
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 25,
        height: 75
    };
    
    let other_rect = Rectangle {
        width: 20,
        height: 40
    };
    
    println!("{:#?}", rect);
    println!("Area: {}", rect.area());
    println!("Can hold other: {}", rect.can_hold(&other_rect));

    println!();

    let square = Rectangle::square(5);

    println!("{:#?}", square);
    println!("Area: {}", square.area());
}
