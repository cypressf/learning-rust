struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    let rectangle = Rectangle {
        width: 30,
        height: 30
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rectangle)
    );
    println!("{}", rectangle.height);
}

fn area(rectangle: Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
