use proconio::input;
use proconio::fastout;
use itertools::Itertools;

#[fastout]
fn main() {
    input! {
        n: i128,
        x: i128,
        a: [i128;n],
    }
    let mut ans: Vec<i128> = Vec::new();
    for i in a.iter() {
        if x == *i {
            continue;
        }
        ans.push(*i);
    }
    println!("{}", ans.iter().map(|x| x.to_string()).join(" "));
}
