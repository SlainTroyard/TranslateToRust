use std::collections::HashMap;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn max_subarray_sum(v: &Vec<i32>, k: i32) -> i64 {
        let mut m: HashMap<i32, i64> = HashMap::new();
        let mut ans = i64::MIN;
        let mut sm: i64 = 0;

        for (i, &num) in v.iter().enumerate() {
            sm += num as i64;
            let cur_sz = i as i32 + 1;
            if cur_sz % k == 0 {
                ans = ans.max(sm);
            }
            let y = cur_sz % k;
            if let Some(&prev_sm) = m.get(&y) {
                ans = ans.max(sm - prev_sm);
                m.insert(y, sm.min(prev_sm));
            } else {
                m.insert(y, sm);
            }
        }
        ans
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Input array size and k value
    let n: usize = lines.next().unwrap()?.parse().unwrap();
    let k: i32 = lines.next().unwrap()?.parse().unwrap();

    // Input array
    let mut nums: Vec<i32> = Vec::with_capacity(n);
    for _ in 0..n {
        nums.push(lines.next().unwrap()?.parse().unwrap());
    }

    // Call the max_subarray_sum function and print the result
    let result = Solution::max_subarray_sum(&nums, k);
    println!("{}", result);

    Ok(())
}