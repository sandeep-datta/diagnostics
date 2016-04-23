#[macro_use]
extern crate diagnostics;

fn main() {
    let x = 10;
    dump!(x);
    let v = vec![1, 2, 3];
    dump!(v);
}