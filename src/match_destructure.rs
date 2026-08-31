pub fn describe() {
    let triple = (0, -2, 3);

    println!("Tell me about {:?}", triple);

    // Match can be used to destructure a tuple
    match triple {
        // Destructure the second and third elements
        (0, z, y) => println!("First is {:?}, `y` is {:?}, and `z` is {:?}", 0, y, z),
        (1, ..) => println!("First is `1` and the rest doesn't matter"),
        (.., 2) => println!("Last is `2` and the rest doesn't matter"),
        (3, .., 4) => println!("First is `3`, last is `4`, and the rest doesn't matter "),
        // `..` can be used to ignore the rest of the tuple
        _ => println!("It doesn't matter what they are"),
        // `_` means don't bind the value to a variable
    }
}
