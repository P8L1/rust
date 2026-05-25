//@ check-pass

#![feature(reborrow)]

use std::marker::{CoerceShared, Reborrow};

struct MyMut<'a>(&'a u8);

impl Reborrow for MyMut<'_> {}

#[derive(Clone, Copy)]
struct MyRef<'a>(&'a u8);

impl<'a> CoerceShared<MyRef<'a>> for MyMut<'a> {}

impl PartialEq<MyRef<'_>> for MyMut<'_> {
    fn eq(&self, _other: &MyRef<'_>) -> bool {
        true
    }
}

fn main() {
    let lhs = MyMut(&0);
    let rhs = MyMut(&0);
    let _ = lhs == rhs;
}
