#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn fits_in(&self, rectangle: &Rectangle) -> bool {
        self.width < rectangle.width && self.height < rectangle.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size}
    }
}

fn main() {
    let rectangle1 = Rectangle {
        width: 30,
        height: 30
    };

    let rectangle2 = Rectangle {
        width: 1,
        height: 29
    };

    // associated functions use ::
    let square = Rectangle::square(5);

    // methods use .
    println!(
        "The area of the rectangle is {} square pixels.",
        rectangle1.area()
    );

    println!(
        "Rectangle: {:?} fits inside rectangle {:?}?\n {}",
        rectangle2, rectangle1, rectangle2.fits_in(&rectangle1)
    );

    println!("Square: {:?}", square);
}
