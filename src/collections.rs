// collections.rs - Vec, HashMap, HashSet, String, Iterators
//
// Run: cargo run -- collections
// These + Option/Result cover 80% of real-world data handling.

use std::collections::{HashMap, HashSet};

#[allow(clippy::vec_init_then_push, clippy::useless_vec)]
pub fn run() {
    println!("--- collections ---");

    // --- Vec<T> (growable array, heap) ---
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    let v2 = vec![4, 5, 6]; // macro
    println!("v={:?}, v2={:?}", v, v2);
    println!("first={:?}, len={}", v.first(), v.len());

    // indexing vs get (get returns Option, no panic)
    // println!("{}", v[10]); // PANICS
    println!("get(10) = {:?}", v.get(10));

    // --- String ops ---
    let mut s = String::from("hello");
    s.push_str(" rust");
    let replaced = s.replace("rust", "world");
    println!("{s} -> {replaced}");
    println!("split: {:?}", "a,b,c".split(',').collect::<Vec<_>>());

    // --- HashMap<K, V> ---
    let mut scores: HashMap<String, u32> = HashMap::new();
    scores.insert("alice".to_string(), 100);
    scores.insert("bob".to_string(), 80);
    // entry API: insert only if absent
    scores.entry("alice".to_string()).or_insert(0);
    scores.entry("cara".to_string()).or_insert(95);
    println!("scores = {:?}", scores);
    println!("bob = {:?}", scores.get("bob"));

    for (k, val) in &scores {
        println!("{k}: {val}");
    }

    // --- HashSet<T> (unique values) ---
    let mut seen = HashSet::new();
    seen.insert(1);
    seen.insert(1);
    seen.insert(2);
    println!("set = {:?}, contains 2 = {}", seen, seen.contains(&2));

    // --- Iterator adapters (lazy! needs collect/for to run) ---
    let nums = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = nums.iter().map(|n| n * 2).collect();
    let evens: Vec<i32> = nums.iter().copied().filter(|n| n % 2 == 0).collect();
    let sum: i32 = nums.iter().sum();
    println!("doubled={:?}, evens={:?}, sum={sum}", doubled, evens);
}
