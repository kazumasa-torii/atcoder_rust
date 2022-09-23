mod ModInt;

pub fn main() {
    let mut a = ModInt::new(1000000000);
    a += ModInt::new(9);
    println!("{}", a.value()); // 2
  
    println!("{}", ModInt::new(2).inverse().value()); // 500000004
    println!("{}", ModInt::new(3).inverse().value()); // 333333336
}