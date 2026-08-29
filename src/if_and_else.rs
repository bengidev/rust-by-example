pub fn describe() {
    let n = 5_i8;

    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }

    let big_n = if n < 10 && n > -10 {
        print!(", and is a small number, increase ten-fold");

        // This expression returns an `i32`.
        10 * n
    } else {
        print!(", and is a big number, halve the number");

        // This expression must return an `i32` as well.
        n / 2
    };

    println!("{} -> {}", n, big_n);
}
