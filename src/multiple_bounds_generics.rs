use std::fmt::{Debug, Display};

pub fn describe() {
    let strings = "words";
    let array = [1, 2, 3];
    let vec = vec![1, 2, 3];

    compare_prints(&strings);
    compare_types(&array, &vec);
}

fn compare_prints<T: Debug + Display>(t: &T) {
    println!("Debug: `{:?}`", t);
    println!("Display: `{}`", t);
}

fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
    println!("t: {:?}", t);
    println!("u: {:?}", u);
}
