use proconio::input;
fn main() {
    input! {
        v: u16,
        t: u16,
        s: u16,
        d: u16,
    }
    if t * v > d || d > s * v {
        print!("{}", "Yes");
    } else {
        print!("{}", "No");
    }
}
