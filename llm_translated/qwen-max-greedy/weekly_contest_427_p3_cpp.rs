use std::collections::BTreeMap;
use std::io::{self, BufRead};

fn max_subarray_sum(v: &Vec<i32>, k: usize) -> i64 {
    let mut m: BTreeMap<usize, i64> = BTreeMap::new();
    let mut ans: i64 = i64::MIN;
    let mut sm: i64 = 0;

    for (i, &val) in v.iter().enumerate() {
        sm += val as i64;
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

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the array and the value of k
    let n: usize = lines.next().unwrap().unwrap().parse().expect("Failed to parse n");
    let k: usize = lines.next().unwrap().unwrap().parse().expect("Failed to parse k");

    // Read the elements of the array
    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to parse array element"))
        .collect();

    // Call the max_subarray_sum function and print the result
    let result = max_subarray_sum(&nums, k);
    println!("{}", result);
}