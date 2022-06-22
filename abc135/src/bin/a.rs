use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
    };

    let ans: i32 = (a + b) % 2;
    if ans == 0 {
        println!("{}", (a + b) / 2);
    } else {
        println!("IMPOSSIBLE");
    }
}
