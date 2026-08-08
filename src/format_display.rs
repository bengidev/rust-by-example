use std::fmt::{self, Display};

pub fn describe() {
    // ===============================================================
    //
    // Display
    //
    // A structure holding two numbers. `Debug` will be derived so the results can
    // be contrasted with `Display`.
    #[derive(Debug)]
    struct MinMax(i64, i64);

    // Implement `Display` for `MinMax`.
    impl Display for MinMax {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            // Use `self.number` to refer to each positional data point.
            write!(f, "({}, {})", self.0, self.1)
        }
    }

    // Define a structure where the fields are nameable for comparison.
    #[derive(Debug)]
    struct Point2D {
        x: f64,
        y: f64,
    }

    // Similarly, implement `Display` for `Point2D`.
    impl Display for Point2D {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            // Customize so only `x` and `y` are denoted.
            write!(f, "x: {}, y: {}", self.x, self.y)
        }
    }

    // ===============================================================
    //
    // Testcase: List
    //
    // Implementing fmt::Display for a structure where the elements must each be handled sequentially is tricky.
    // The problem is that each write! generates a fmt::Result. Proper handling of this requires dealing with all the results.
    // Rust provides the ? operator for exactly this purpose.
    //

    // Define a structure named `List` containing a `Vec`.
    struct List(Vec<i32>);

    impl Display for List {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            // Create a reference to the Vec<i32> stored in the List struct.
            let vec = &self.0;

            write!(f, "[")?;

            // Iterate over `v` in `vec` while enumerating the iteration
            // index in `index`.
            for (index, v) in vec.iter().enumerate() {
                // For every element except the first, add a comma.
                // Use the ? operator to return on errors.
                if index != 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}: {}", v, v + 1)?;
            }

            // Close the opened bracket and return a fmt::Result value.
            write!(f, "]")
        }
    }

    // ===============================================================
    //
    // Display
    //
    let minMax = MinMax(0, 14);

    println!("Compare structures: ");
    println!("Display: {}", minMax);
    println!("Debug: {:?}", minMax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!(
        "The big range is {big} and the small is {small}",
        small = small_range,
        big = big_range
    );

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points: ");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    // ===============================================================
    //
    // Testcase: List
    //
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
