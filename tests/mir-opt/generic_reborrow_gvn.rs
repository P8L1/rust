//@ test-mir-pass: GVN
//@ compile-flags: -Cpanic=abort

#![feature(reborrow)]
#![allow(dead_code)]

use std::marker::Reborrow;

struct Wrap<'a>(&'a mut u32);

impl Reborrow for Wrap<'_> {}

fn opaque(_: Wrap<'_>) {}

// EMIT_MIR generic_reborrow_gvn.mutable.GVN.diff
fn mutable(x: Wrap<'_>) {
    // CHECK-LABEL: fn mutable(
    // CHECK: [[reborrow:_.*]] = copy _1;
    let y: Wrap<'_> = x;
    opaque(y);
}

fn main() {}
