fn main() {
	// a mutable variable
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // an immutable variable
    let y = 10;
    println!("The value of y is {} and cannot change", y);

    // a shadowed variable
    let y = y * 4 + 2;
    println!("The value of y was redeclared and shadowed as {} now", y);

    // shadowed variables can change type
    let spaces = "     ";
    let spaces = spaces.len();
    println!("`spaces` changed from a string to a count ({})", spaces);


    // a constant
    const Z: u32 = 100_000;
    println!("The value of Z is {} and is fixed at compile time", Z);
}
