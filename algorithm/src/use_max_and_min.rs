use std::cmp;

pub fn main() {
    println!("{}", cmp::min(1, 2));
    println!("{}", cmp::min(1000, -1));

    println!("{}", cmp::max(1, 2));
    println!("{}", cmp::max(1, -1));
}