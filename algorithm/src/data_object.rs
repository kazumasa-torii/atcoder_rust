use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::collections::HashSet;
use std::collections::HashMap;

pub fn main() {
    // 可変長型の配列
    let mut v = vec![0, 1, 2, 3, 4];
    println!("{}", v.len());
    println!("{:?}", v);
    
    v[2] = 5;
    println!("{:?}", v);
    
    // stack型の配列
    let mut stack = vec![];
    stack.push(9);
    stack.push(1);
    stack.push(11);
    
    println!("{:?}", stack);
    println!("{:?}", stack.pop());
    println!("{:?}", stack);
    
    // queue型の配列
    let mut que = VecDeque::new();
    que.push_back(0);
    que.push_back(1);
    que.push_back(2);
    
    println!("{:?}", que);
    println!("{:?}", que.pop_front());
    println!("{:?}", que);
    
    que.push_front(1);
    que.push_front(2);
    que.push_front(3);
    
    println!("{:?}", que);
    println!("{:?}", que.pop_back());
    println!("{:?}", que);

    // 優先度付きキュー 大きい順に取得できる
    let mut que = BinaryHeap::new();
    que.push(3);
    que.push(0);
    que.push(1);
    que.push(2);
    
    println!("{:?}", que);
    println!("{:?}", que.pop());
    println!("{:?}", que.pop());
    que.push(4);
    println!("{:?}", que.pop());
    println!("{:?}", que.pop());

    // 優先度付きキュー 小さい順で取得できる
    let mut que = BinaryHeap::new();
    que.push(Reverse(3));
    que.push(Reverse(0));
    que.push(Reverse(1));
    que.push(Reverse(2));
    
    println!("{:?}", que);
    println!("{:?}", que.pop());
    println!("{:?}", que.pop());
    que.push(Reverse(4));
    println!("{:?}", que.pop());
    println!("{:?}", que.pop());
    
    // set型 重複排除みたいなやつ
    let mut set = HashSet::new();

    set.insert(0);
    set.insert(1);
    set.insert(2);

    println!("{}", set.contains(&0));
    println!("{}", set.contains(&3));

    set.remove(&0);
    println!("{}", set.contains(&0));

    // map 一つのキーに対して一つの値みたいなやつ
    let mut map = HashMap::new();

    map.insert("zero", 0);
    map.insert("one", 1);
    map.insert("two", 2);

    println!("{:?}", map.get("zero")); // Some(0)
    println!("{:?}", map.get("three")); // None
}