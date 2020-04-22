#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rectangle = Rectangle {
        width: 30,
        height: 30
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rectangle.area()
    );

    // we still have ownership of rectangle
    println!("{:?}", rectangle);
}
