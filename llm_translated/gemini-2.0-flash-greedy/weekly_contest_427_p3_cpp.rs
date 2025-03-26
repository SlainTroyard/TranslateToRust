use std::collections::HashMap;
use std::io;
use std::cmp::{max, min};

fn max_subarray_sum(v: &Vec<i32>, k: i32) -> i64 {
    let mut m: HashMap<i32, i64> = HashMap::new();
    let mut ans: i64 = i64::min_value();
    let mut sm: i64 = 0;
    for i in 0..v.len() {
        sm += v[i] as i64;
        let cur_sz = i + 1;
        if cur_sz as i32 % k == 0 {
            ans = max(ans, sm);
        }
        let y = (cur_sz as i32 % k);
        if m.contains_key(&y) {
            ans = max(ans, sm - m[&y]);
            m.insert(y, min(m[&y], sm));
        } else {
            m.insert(y, sm);
        }
    }
    ans
}

fn main() {
    let mut n_str = String::new();
    io::stdin().read_line(&mut n_str).expect("Failed to read line");
    let n: i32 = n_str.trim().parse().expect("Invalid input");

    let mut k_str = String::new();
    io::stdin().read_line(&mut k_str).expect("Failed to read line");
    let k: i32 = k_str.trim().parse().expect("Invalid input");

    let mut nums_str = String::new();
    io::stdin().read_line(&mut nums_str).expect("Failed to read line");
    let nums: Vec<i32> = nums_str
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid number"))
        .collect();

    let result = max_subarray_sum(&nums, k);
    println!("{}", result);
}