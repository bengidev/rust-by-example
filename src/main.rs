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

mod arrays_and_slices;
mod casting;
mod const_global_static;
mod declare_first;
mod enum_c_like;
mod enums;
mod format_display;
mod formatting;
mod freezing;
mod linked_list_enum;
mod literals;
mod literals_and_operators;
mod mutability;
mod primitives;
mod prints;
mod scope_and_shadowing;
mod structs;
mod tuples;
mod use_use;
mod variable_bindings;

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
    arrays_and_slices::describe();
    structs::describe();
    enums::describe();
    use_use::describe();
    enum_c_like::describe();
    linked_list_enum::describe();
    const_global_static::describe();
    variable_bindings::describe();
    mutability::describe();
    scope_and_shadowing::scope_and_shadowing();
    declare_first::describe();
    freezing::describe();
    casting::describe();
    literals::describe();
}
