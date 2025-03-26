use std::collections::HashMap;
use std::io::{self, BufRead};

fn max_subarray_sum(v: Vec<i32>, k: i32) -> i64 {
    let mut m: HashMap<i32, i64> = HashMap::new();
    let mut ans = i64::MIN;
    let mut sm = 0i64;

    for (i, &num) in v.iter().enumerate() {
        sm += num as i64;
        let cur_sz = (i + 1) as i32;
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

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read n and k from the first line
    let n: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let k: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read the array from the second line
    let nums: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Call the max_subarray_sum function and print the result
    let result = max_subarray_sum(nums, k);
    println!("{}", result);
}