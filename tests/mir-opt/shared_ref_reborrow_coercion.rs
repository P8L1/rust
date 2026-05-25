//@ skip-filecheck
// EMIT_MIR shared_ref_reborrow_coercion.coerce_arg.built.after.mir

unsafe extern "Rust" {
    fn take(_: &u32);
}

fn coerce_arg<'a, 'b>(x: &'a u32, y: &'b u32, flag: bool)
where
    'b: 'a,
{
    let z = if flag { x } else { y };
    unsafe { take(z) };
}

fn main() {}
