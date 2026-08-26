pub fn describe() {}

#[derive(Debug)]
struct EvenNumber(i32);

// Similar to From and Into, TryFrom and TryInto are generic traits for converting between types.
// Unlike From/Into, the TryFrom/TryInto traits are used for fallible conversions, and as such, return Results.
//
impl TryFrom<i32> for EvenNumber {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err("EvenNumber didn't support your input!")
        }
    }
}
