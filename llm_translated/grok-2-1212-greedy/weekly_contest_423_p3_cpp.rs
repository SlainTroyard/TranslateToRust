use std::collections::BTreeMap;
use std::io::{self, BufRead};

struct Solution {
    mod_val: i64,
}

impl Solution {
    fn new() -> Self {
        Solution { mod_val: 1_000_000_007 }
    }

    fn sum_of_good_subsequences(&self, nums: Vec<i32>) -> i32 {
        let mut cnt = BTreeMap::new();
        let mut sum = BTreeMap::new();

        for &num in &nums {
            let num = num as i64;
            let prev_cnt = *cnt.get(&(num - 1)).unwrap_or(&0);
            let next_cnt = *cnt.get(&(num + 1)).unwrap_or(&0);
            let current_cnt = (prev_cnt + next_cnt + 1) % self.mod_val;
            cnt.entry(num).and_modify(|e| *e = (*e + current_cnt) % self.mod_val).or_insert(current_cnt);

            let prev_sum = *sum.get(&(num - 1)).unwrap_or(&0);
            let next_sum = *sum.get(&(num + 1)).unwrap_or(&0);
            let current_sum = (prev_sum + next_sum) % self.mod_val;
            sum.entry(num).and_modify(|e| *e = (*e + current_sum) % self.mod_val).or_insert(current_sum);

            let contribution = ((prev_cnt + next_cnt + 1) % self.mod_val * num % self.mod_val) % self.mod_val;
            sum.entry(num).and_modify(|e| *e = (*e + contribution) % self.mod_val).or_insert(contribution);
        }

        let mut res = 0;
        for (_, &val) in &sum {
            res = (res + val) % self.mod_val;
        }

        res as i32
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Reading input
    let n: usize = lines.next().unwrap()?.parse().unwrap();
    let nums: Vec<i32> = lines.next().unwrap()?.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Create an object of Solution
    let solution = Solution::new();

    // Call the method and get the result
    let result = solution.sum_of_good_subsequences(nums);

    // Output the result
    println!("{}", result);

    Ok(())
}