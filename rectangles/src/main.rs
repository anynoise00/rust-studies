#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        self.width * 2 + self.height * 2
    }

    fn can_hold(&self, other_rect: &Rectangle) -> bool {
        other_rect.width <= self.width && other_rect.height <= self.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 40,
    };

    let rect2 = Rectangle {
        width: 20,
        height: 30,
    };

    let rect3 = Rectangle::square(50);

    println!("rect1 is {:#?}", rect1);
    println!("The area of rect1 is: {}", rect1.area());
    println!("The perimeter of rect1 is: {}", rect1.perimeter());
    println!();
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}