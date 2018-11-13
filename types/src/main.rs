fn main() {
    let heart_eyed_cat: char = '😻';
    println!("`char` can be any Unicode Scalar Value: {}", heart_eyed_cat);

    let values: (i32, f64, &str) = (255, 4.2, "hello 🤓");
    let (integer, float, string) = values;
    println!("tuples can be deconstructed ({}, {}, {})", integer, float, string);
    println!("also referenced by index {}", values.2);

    let array: [i32; 3] = [1, 2, 3];
    println!("arrays are a fixed length, and allocated on the stack {}", array[2]);
}
