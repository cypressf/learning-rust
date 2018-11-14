fn main() {
    let my_name = String::from("Cypress"); // my_name comes into scope
    let my_name_again = take_and_give_back(my_name); // my_named moved into function,
                                                     // which moves its return value
                                                     // into my_name_again
    println!("Moved string {}", my_name_again);

    let length = calculate_length(&my_name_again); // my_name_again isn't moved if
                                                   // passed by reference

    println!("length of string {}: {}", my_name_again, length);

    let mut my_name_mutable = my_name_again;
    modify_reference(&mut my_name_mutable);

    println!("{}", my_name_mutable);

    multiple_mut_references();
}

fn take_and_give_back(a_string: String) -> String {
    a_string
}

fn calculate_length(a_string: &String) -> usize {
    a_string.len() // cannot modify a_string because it is borrowed
                   // (references are immutable)
}

fn modify_reference(a_string: &mut String) {
    a_string.push_str(" The ships hung in the sky in much the same way that bricks don't");
}

fn multiple_mut_references() {
    let mut s = String::from("hello");
    {
        let _r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let _r2 = &mut s;
}