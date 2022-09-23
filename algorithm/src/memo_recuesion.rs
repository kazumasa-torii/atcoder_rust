/**
 * rustはグローバル変数を扱えないからキチンと引数に渡して対応する
 */

use std::collections::HashMap;

fn fib(n: usize, memo: &mut HashMap<usize, u128>) -> u128 {
    if let Some(value) = memo.get(&n) {
        return *value;
    }

    let res = match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 2, memo) + fib(n - 1, memo)
    };

    memo.insert(n, res);
    res
}

fn main(){
    let mut memo = HashMap::new();
    println!("{}", fib(100, &mut memo)); // 354224848179261915075
}