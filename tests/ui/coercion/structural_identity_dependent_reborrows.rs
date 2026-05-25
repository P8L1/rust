//@ edition: 2024
//@ check-pass

// We emit shared reborrow coercions even when the resulting type is
// structurally identical to the source type. This keeps capture analysis
// from depending on whether the coercion changed the reference type.

fn foo<'a>(b: &'a ()) -> impl Fn() {
    || {
        expected::<&()>(b);
    }
}

fn bar<'a>(b: &'a ()) -> impl Fn() {
    || {
        expected::<&'a ()>(b);
    }
}

fn expected<T>(_: T) {}

fn main() {}
