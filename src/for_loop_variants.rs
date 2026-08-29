pub fn describe() {
    // The for in construct can be used to iterate through an Iterator.
    // One of the easiest ways to create an iterator is to use the range notation a..b.
    // This yields values from a (inclusive) to b (exclusive) in steps of one.
    //
    // `n` will take the values: 1, 2, ..., 100 in each iteration
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // Alternatively, a..=b can be used for a range that is inclusive on both ends.
    //
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // Just remember that even though you can compile the code when a>b, the loop gets never executed.
    //
    for i in 10..1 {
        println!("fizzbuzz");
    }

    // If you want to count down, you need to use .rev() instead
    for i in (1..11).rev() {
        println!("fizzbuzz of: {}", i);
    }

    // iter - This borrows each element of the collection through each iteration.
    // Thus leaving the collection untouched and available for reuse after the loop.
    //
    let first_names = vec!["Bob", "Beng", "Frank", "Ferris"];

    for name in first_names.iter() {
        match name {
            &"Beng" => println!("There is a rustacean among us!"),
            _ => println!("Hello, {}", name),
        }
    }

    println!("first_names: {:?}", first_names);

    // into_iter - This consumes the collection so that on each iteration the exact data is provided.
    // Once the collection has been consumed it is no longer available for reuse as it has been ‘moved’ within the loop.
    //
    let second_names = vec!["Bob", "Beng", "Frank", "Ferris"];

    for name in second_names.into_iter() {
        match name {
            "Beng" => println!("There is a rustacean among us!"),
            _ => println!("Hello, {}", name),
        }
    }

    // `names` has been 'moved' and can no longer be used.
    // Try uncommenting the line below to see the compiler error:
    // println!("second_names: {:?}", second_names);

    //iter_mut - This mutably borrows each element of the collection, allowing for the collection to be modified in place.
    let mut third_names = vec!["Bob", "Beng", "Frank", "Ferris"];

    for name in third_names.iter_mut() {
        *name = match name {
            &mut "Beng" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("third_names: {:?}", third_names);
}
