use std::{fmt, num::ParseIntError, str::FromStr};

pub fn describe() {
    // to string
    let circle = Circle { radius: 6 };
    println!("To string: {}", circle.to_string());

    // from string
    let radius_new = "      3";
    let circle_new: Circle = radius_new.parse().unwrap();
    println!("From string: {:?}", circle_new);
}

#[derive(Debug)]
struct Circle {
    radius: i32,
}

// To convert any type to a String is as simple as implementing the ToString trait for the type.
// Rather than doing so directly, you should implement the fmt::Display trait which automatically
// provides ToString and also allows printing the type as discussed in the section on print!.
//
impl fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

// It’s useful to convert strings into many types, but one of the more common string operations is to convert them from string to number.
// The idiomatic approach to this is to use the parse function and either to arrange for type inference or to specify the type to parse using
// the ‘turbofish’ syntax. Both alternatives are shown in the following example.
//
impl FromStr for Circle {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().parse() {
            Ok(num) => Ok(Circle { radius: num }),
            Err(e) => Err(e),
        }
    }
}
