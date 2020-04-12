fn main() {
    struct User {
        name: String,
        email: String,
        sign_in_count: u64,
        active: bool
    };

    let mut cypress: User = User {
        name: String::from("Cypress"),
        email: String::from("example@example.com"),
        sign_in_count: 0,
        active: true
    };

    println!("{}", cypress.name);

    cypress.name = String::from("Cypress Frankenfeld");

    println!("{}", cypress.name);

    // Field init shorthand: we don't have to write
    // name: name,
    let name = String::from("Bob");
    let user1 = User {
        name,
        email: String::from("example@example.com"),
        sign_in_count: 0,
        active: true
    };

    // Struct update syntax to create an instance from a previous one

    let user2 = User {
        email: String::from("example2@example.com"),
        ..user1
    };

    // Tuple structs for when naming fields would be unnecessary, but you want the
    // type name defined

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let color = Color(0,0,0);
    let point = Point(1,1,1);
}
