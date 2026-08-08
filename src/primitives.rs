pub fn describe() {
    // ===============================================================
    //
    // Primitives
    //
    // Variables can always be type annotated.
    // Numbers may additionally be annotated via a suffix or by default.
    // Integers default to i32 and floats to f64.
    // Note that Rust can also infer types from context.
    //

    // Variables can be type annotated.
    let logical: bool = true;
    println!("logical: {}", logical);

    let a_float: f64 = 1.0; // Regular annotation
    println!("a_float: {}", a_float);

    let an_integer = 5i32; // Suffix annotation
    println!("an_integer: {}", an_integer);

    // Or a default will be used.
    let default_float = 3.0; // `f64`
    println!("default_float: {}", default_float);

    let default_integer = 7; // `i32`
    println!("default_integer: {}", default_integer);

    // A type can also be inferred from context.
    let mut inferred_type = 12; // Type i64 is inferred from another line.
    inferred_type = 4294967296i64;
    println!("inferred_type: {}", inferred_type);

    // A mutable variable's value can be changed.
    let mut mutable = 12; // Mutable `i32`
    mutable = 21;
    println!("mutable: {}", mutable);

    // Error! The type of a variable can't be changed.
    // mutable = true;

    // Variables can be overwritten with shadowing.
    let mutable = true;
    println!("mutable: {}", mutable);

    /* Compound types - Array and Tuple */

    // Array signature consists of Type T and length as [T; length].
    let my_array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("my_array: {:?}", my_array);

    // Tuple is a collection of values of different types
    // and is constructed using parentheses ().
    let my_tuple = (5u32, 1u8, true, -5.04f32);
    println!("my_tuple: {:?}", my_tuple);
}
