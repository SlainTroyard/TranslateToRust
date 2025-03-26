use std::collections::BTreeMap;
use std::io::{self, BufRead};

fn max_subarray_sum(v: &Vec<i32>, k: i32) -> i64 {
    let mut m: BTreeMap<i32, i64> = BTreeMap::new();
    let mut ans: i64 = i64::MIN;
    let mut sm: i64 = 0;

    for (i, &val) in v.iter().enumerate() {
        sm += val as i64;
        let cur_sz = (i + 1) as i32;
        if cur_sz % k == 0 {
            ans = ans.max(sm);
        }
        let y = cur_sz % k;
        if let Some(&prev_val) = m.get(&y) {
            ans = ans.max(sm - prev_val);
            m.insert(y, prev_val.min(sm));
        } else {
            m.insert(y, sm);
        }
    }

    ans
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Parse n and k
    let n: usize = lines.next().unwrap().unwrap().trim().parse().expect("Failed to parse n");
    let k: i32 = lines.next().unwrap().unwrap().trim().parse().expect("Failed to parse k");

    // Parse the array
    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to parse array element"))
        .collect();

    // Call the max_subarray_sum function and print the result
    let result = max_subarray_sum(&nums, k);
    println!("{}", result);
}