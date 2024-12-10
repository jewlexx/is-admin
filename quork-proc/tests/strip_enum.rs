#![allow(dead_code)]

use std::fmt::Display;

use strum::{Display, EnumIter, IntoEnumIterator};

use quork_proc::Strip;

pub fn enum_to_string<T: IntoEnumIterator + Display>() -> String {
    T::iter().map(|v| v.to_string()).collect::<String>()
}

struct DummyStruct;

#[derive(Strip)]
#[stripped_meta(derive(EnumIter, Display))]
#[stripped_meta(strum(serialize_all = "kebab-case"))]
enum EnumWithData {
    Test1(DummyStruct),
    Test2(DummyStruct),
}

#[test]
fn has_all_variants() {
    let variants = enum_to_string::<EnumWithDataStripped>();

    assert_eq!(variants, "test1test2");
}

#[derive(Strip)]
#[stripped_meta(derive(EnumIter, Display))]
#[stripped_meta(strum(serialize_all = "kebab-case"))]
enum EnumExclude {
    Test1(DummyStruct),
    #[stripped_ignore]
    Test2(DummyStruct),
    Test3(DummyStruct),
}

#[test]
fn excludes_no_hook_variant() {
    let variants = enum_to_string::<EnumExcludeStripped>();

    assert_eq!(variants, "test1test3");
}
