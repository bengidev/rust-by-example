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
// Formatting
//
// This formatting functionality is implemented via traits,
// and there is one trait for each argument type.
// The most common formatting trait is Display,
// which handles cases where the argument type is left unspecified: {} for instance.

struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl Display for City {
    // `f` is a buffer, and this method must write the formatted string into it.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // `write!` is like `format!`, but it will write the formatted string
        // into a buffer (the first argument).
        write!(
            f,
            "{}: {:.3}°{} {:.3}°{}",
            self.name,
            self.lat.abs(),
            lat_c,
            self.lon.abs(),
            lon_c
        )
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
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

    // ===============================================================
    //
    // Testcase: List
    //
    let v = List(vec![1, 2, 3]);
    println!("{}", v);

    // ===============================================================
    //
    // Formatting
    //
    for city in [
        City {
            name: "Dublin",
            lat: 53.347778,
            lon: -6.259722,
        },
        City {
            name: "Oslo",
            lat: 59.95,
            lon: 10.75,
        },
        City {
            name: "Vancouver",
            lat: 49.25,
            lon: -123.1,
        },
    ] {
        println!("{}", city);
    }
    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ] {
        // Switch this to use {} once you've added an implementation
        // for fmt::Display.
        println!("{:?}", color);
    }
}
