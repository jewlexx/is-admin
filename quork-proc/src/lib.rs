//! Quork procedural macros crate

#![warn(clippy::pedantic)]
#![warn(missing_docs)]

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use syn::{parse_macro_input, DeriveInput, LitStr};

mod const_str;
mod enum_list;
mod from_tuple;
mod new;
mod strip;
mod strip_enum;
mod time_fn;

#[macro_use]
extern crate quote;

/// Create an additional enum with all values stripped
#[proc_macro_derive(Strip, attributes(stripped_meta, stripped))]
#[proc_macro_error]
pub fn strip_enum(input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);

    strip_enum::strip_enum(&mut ast).into()
}

/// Implement [`quork::ListVariants`] for enums
#[proc_macro_derive(ListVariants)]
#[proc_macro_error]
pub fn derive_enum_list(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    enum_list::enum_list(&ast).into()
}

// TODO: Add heck for case renaming like strum
/// Implement `const_to_string` for enum variants.
///
/// Converts an enum variant to a string literal, within a constant context.
#[proc_macro_derive(ConstStr)]
pub fn derive_const_str(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    const_str::derive(&ast).into()
}

/// Implement `new` fn for structs
///
/// Will follow the form of `new(field: Type, ...) -> Self`, where all fields are required.
#[proc_macro_derive(New)]
#[proc_macro_error]
pub fn derive_new(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    new::derive(&ast).into()
}

/// Implement the [`std::convert::From`] trait for converting tuples into tuple structs
#[proc_macro_derive(FromTuple)]
#[proc_macro_error]
pub fn derive_from_tuple(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    from_tuple::derive(&ast).into()
}

/// Time a given function
///
/// Measures the start and finish times of the function, and prints them at the end of the function.
///
/// You can pass "s", "ms", "ns"
#[proc_macro_attribute]
#[proc_macro_error]
pub fn time(args: TokenStream, input: TokenStream) -> TokenStream {
    let args_str = args.to_string();
    let fmt = match args_str.as_str() {
        "ms" | "milliseconds" => time_fn::TimeFormat::Milliseconds,
        "ns" | "nanoseconds" => time_fn::TimeFormat::Nanoseconds,
        _ => time_fn::TimeFormat::Seconds,
    };
    time_fn::attribute(fmt, &input.into()).into()
}

/// Strip whitespace from the right of a string literal on each line
#[proc_macro]
pub fn strip_lines(input: TokenStream) -> TokenStream {
    let literal = parse_macro_input!(input as LitStr);

    strip::funclike(&literal, &strip::Alignment::None).into()
}

/// Strip whitespace from the left and right of a string literal on each line
#[proc_macro]
pub fn rstrip_lines(input: TokenStream) -> TokenStream {
    let literal = parse_macro_input!(input as LitStr);

    strip::funclike(&literal, &strip::Alignment::Right).into()
}

/// Strip whitespace from the left of a string literal on each line
#[proc_macro]
pub fn lstrip_lines(input: TokenStream) -> TokenStream {
    let literal = parse_macro_input!(input as LitStr);

    strip::funclike(&literal, &strip::Alignment::Left).into()
}
