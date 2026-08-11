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

mod format_display;
mod formatting;
mod literals_and_operators;
mod primitives;
mod prints;
mod tuples;

use std::fmt::{self, Display};

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

    prints::describe();
    format_display::describe();
    formatting::describe();
    primitives::describe();
    literals_and_operators::describe();
    tuples::describe();
}
