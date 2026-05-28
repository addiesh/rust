//@check-pass
#![feature(closures_of_mass_destruction)]

#[allow(unused)]
fn closure_bounded<'a, T>(
    count: &'a mut usize,
) -> impl for<'b> FnMut(&'b T) + 'a {
    move |x| {
        *count += 1;
    }
}

fn main() {}
