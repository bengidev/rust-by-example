pub fn describe() {
    let test = "10".parse::<u8>().ok().unwrap_or_default();
    println!("test value: {:?}", test);

    // From
    let num = Number::from(30);
    println!("My number is: {:?}", num);

    // Into
    let int = 5;
    let num_into: Number = int.into();
    println!("My number into is: {:?}", num_into);
}

// From
//
// The From trait allows for a type to define how to create itself from another type,
// hence providing a very simple mechanism for converting between several types.
//
#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

// Into
//
// The Into trait is simply the reciprocal of the From trait.
// It defines how to convert a type into another type.
//
// Prefer using [Into] over [From] when specifying trait bounds on a generic function to ensure that
// types that only implement [Into] can be used as well.
//
// impl Into<Number> for i32 {
//     fn into(self) -> Number {
//         Number { value: self }
//     }
// }
