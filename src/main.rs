// Debug
//
// This structure cannot be printed either with `fmt::Display` or
// with `fmt::Debug`.
// struct UnPrintable(i32);
//
// The `derive` attribute automatically creates the implementation
// required to make this `struct` printable with `fmt::Debug`.
// #[derive(Debug)]
// struct DebugPrintable(i32);

use std::fmt::{self, Display};

// Derive the `fmt::Debug` implementation for `Structure`. `Structure`
// is a structure which contains a single `i32`.
#[derive(Debug)]
struct Structure(i32);

// Put a `Structure` inside of the structure `Deep`. Make it printable
// also.
#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

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

fn main() {
    // Comments
    //
    //code comments

    /// doc comments

    // ===============================================================

    // Hello worlds!
    //
    println!("Hello, world!");
    println!("I'm a Rustacean!");

    // ===============================================================

    // Formatted print
    //
    // In general, the `{}` will be automatically replaced with any
    // // arguments. These will be stringified.
    println!("{} days", 31);

    // Positional arguments can be used. Specifying an integer inside `{}`
    // determines which additional argument will be replaced. Arguments start
    // at 0 immediately after the format string.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments.
    println!(
        "{subject} {verb} {object}",
        object = "|the lazy dog|",
        subject = "|the quick brown fox|",
        verb = "|jumps over|"
    );

    // ===============================================================
    //
    // Debug
    //
    // Printing with `{:?}` is similar to with `{}`.
    println!("{:?} months in a year", 12);
    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );

    // `Structure` is printable!
    println!("Now {:?} will print!", Structure(3));

    // fmt::Debug definitely makes this printable but sacrifices some elegance. Rust also provides “pretty printing” with {:#?}.
    let name = "Peter";
    let age = 28;
    let peter = Person { name, age };
    // pretty print
    println!("{:#?}", peter);

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
}
