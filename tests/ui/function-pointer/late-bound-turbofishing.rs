//@check-pass
#![feature(late_bound_turbofishing)]

fn foo_early<'a: 'a, T>(x: &'a T) -> &'a T {
    x
}

fn foo_late<'a, T>(x: &'a T) -> &'a T {
    x
}

const fn require_static<T: 'static>(x: &T) {}

fn main() {
    let spec = foo_early::<'static, ()>;
    require_static(&spec);
    let spec = foo_late::<'static, ()>;
    require_static(&spec);
}
