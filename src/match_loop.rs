pub fn describe() {
    let number = 13_u8;

    println!("Tell me about: {}", number);

    match number {
        // Match a single value
        1 => println!("One"),
        // Match several values
        2 | 3 | 5 | 7 | 11 | 13 => println!("This is a prime"),
        // Match an inclusive range
        13..=19 => println!("A teen"),
        // Handle the rest of cases
        _ => println!("Ain't special"),
    }

    let boolean = true;
    // Match is an expression too
    let binary = match boolean {
        true => 0,
        false => 1,
    };

    println!("{} -> {}", boolean, binary);
}
