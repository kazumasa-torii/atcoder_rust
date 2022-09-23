use proconio::input;
mod BinarySearch;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        mut b: [usize; n],
        mut c: [usize; n],
    }

    a.sort();
    b.sort();
    c.sort();

    let mut ans = 0;
    for i in 0..n {
        ans += a.lower_bound(&b[i]) * (n - c.upper_bound(&b[i]));
    }

    println!("{}", ans);
}
