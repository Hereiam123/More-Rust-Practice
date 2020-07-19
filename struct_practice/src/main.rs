#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

//Implementing a function method on the
//Rectangle struct
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    //Can one rectangle hold another rectangle
    //within it?
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    //Associated functions
    //Not specified to instance of struct
    //Returns instance of Rectangle that is a square
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 40,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        //Auto dereferencing works in rust
        //e.g. below is equivalent to &rect.area()
        rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    println!("Associated function call {:?}", Rectangle::square(5));
    //println!("{:?}", rect1);
}
