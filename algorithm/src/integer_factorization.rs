/**
 *  mapのentryを使うと指数を簡単に記録できます。
 *  BTreeMapで実装していますが、HashMapでも問題無いです。
 */

use std::collections::BTreeMap;

pub fn integer_factorization(mut n: usize) -> BTreeMap<usize, usize> {
    let mut map = BTreeMap::new();

    let mut i = 2;
    while i * i <= n {
        while n % i == 0 {
            *map.entry(i).or_insert(0) += 1;
            n /= i;
        }
        i += 1;
    }

    if n != 0 && n != 1 {
        *map.entry(n).or_insert(0) += 1;
    }

    map
}

fn main() {
    println!("{:?}", integer_factorization(12)); // {2: 2, 3: 1}
    println!("{:?}", integer_factorization(100)); // {2: 2, 5: 2}
}