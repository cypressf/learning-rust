fn find_largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    print!(
        "{}",
        find_largest(&[String::from("a"), String::from("b"), String::from("c")])
    );
}
