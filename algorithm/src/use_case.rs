use itertools::Itertools;
use proconio::input;
use proconio::source::auto::AutoSource;

fn main() {
    let args = std::args().collect_vec();
    let args_str = args[1..].join(" ");
    let source = AutoSource::from(&args_str[..]);

    input! {
        from: source,
        arg1: usize,
        arg2: f64,
        args_ex: [usize; args.len() - 3],
    }

    println!("{} {} .. {:?}", arg1, arg2, args_ex);

    return;
}
