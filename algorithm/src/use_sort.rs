pub fn main() {
    // 通常のsort
    let mut v = vec![3, 2, 3, 4, 5, 1, 3, 5];
    v.sort();
    println!("{:?}", v);
    
    // sortをreverse
    let mut v2 = vec![3, 2, 3, 4, 5, 1, 3, 5];
    v2.sort_by(|a, b| a.cmp(b).reverse());
    println!("{:?}", v2);
    
    // 少数でのsort
    let mut v3 = vec![3.0, 1.0, 4.0, 1.0, 5.0];
    v3.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("{:?}", v3);
    
    // 各tupleでsort
    let mut v4 = vec![(3, 2, 3), (4, 5, 1), (3, 5, 4)];
    
    // 各tupleの0番目の要素をもとにsort
    v4.sort_by_key(|x| x.0);
    println!("{:?}", v4);
    
    // 各tupleの1番目の要素をもとにsort
    v4.sort_by_key(|x| x.1);
    println!("{:?}", v4);
    
    // 各tupleの2番目の要素をもとにsort
    v4.sort_by_key(|x| x.2);
    println!("{:?}", v4);
    
    // 要素を反転させる
    let mut v5 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    v5.reverse();
    println!("{:?}", v5);
    
    // 重複の排除(連続した値なので必ずsortする)
    let mut v6 = vec![3, 2, 3, 4, 5, 1, 3, 5];
    v6.sort();
    v6.dedup();
    println!("{:?}", v6);
}