use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first line for n and k
    let first_line = lines.next().unwrap().unwrap();
    let mut iter = first_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: usize = iter.next().unwrap().parse().unwrap();

    // Read the second line for the array elements
    let second_line = lines.next().unwrap().unwrap();
    let nums: Vec<i32> = second_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Call the max_subarray_sum function and print the result
    let result = max_subarray_sum(&nums, k);
    println!("{}", result);
}

fn max_subarray_sum(v: &[i32], k: usize) -> i64 {
    let mut m: HashMap<usize, i64> = HashMap::new();
    let mut ans = i64::MIN;
    let mut sm: i64 = 0;

    for (i, &num) in v.iter().enumerate() {
        sm += num as i64;
        let cur_sz = i + 1;
        if cur_sz % k == 0 {
            ans = ans.max(sm);
        }
        let y = cur_sz % k;
        if let Some(&prev_sm) = m.get(&y) {
            ans = ans.max(sm - prev_sm);
            m.insert(y, prev_sm.min(sm));
        } else {
            m.insert(y, sm);
        }
    }

    ans
}