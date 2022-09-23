use std::mem;

pub fn main() {
    let mut a = 1;
    let mut b = 2;
    println!("a : {}, b : {}", a, b);
    mem::swap(&mut a, &mut b);
    println!("a : {}, b : {}", a, b);
}