fn main() {
    let original_number = 5;
    let new_number = plus_one(original_number);
    println!("Original number = {}, number + 1 = {}",
        original_number,
        new_number);
    if new_number > 6 {
        println!("It was higher than 6");
    } else if new_number == 6 {
        println!("It was 6");
    } else {
        println!("It was lower than 6")
    }
}

fn plus_one(x: i32) -> i32 {
    x + 1
}