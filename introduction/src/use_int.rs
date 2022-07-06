use num::Complex;

pub fn main() {
    let twenty: i32 = 20;
    let twenty_one: i32 = 21;
    let twenty_two = 22i32;
    let addition = twenty + twenty_one + twenty_two;

    println!(
        "{} + {} + {} = {}",
        twenty, twenty_one, twenty_two, addition
    );

    let forty_twos = [42.0, 42f32, 42.0_f32];

    println!("{:02}", forty_twos[0]);

    // 0b => binary 0o => octal 0x => hexadecimal
    let three = 0b11;
    let thirty = 0o32;
    let three_hundred = 0x12C;

    println!("base 10: {} {} {}", three, thirty, three_hundred);
    println!("base 2: {:b} {:b} {:b}", three, thirty, three_hundred);
    println!("base 8: {:o} {:o} {:o}", three, thirty, three_hundred);
    println!("base 16: {:x} {:x} {:x}", three, thirty, three_hundred);

    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;

    println!("{} + {}i", result.re, result.im);
}
