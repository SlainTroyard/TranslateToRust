use std::collections::HashMap;
use std::io::{self, BufRead};

struct Solution {
    mod_val: i32,
}

impl Solution {
    fn new() -> Self {
        Solution { mod_val: 1_000_000_007 }
    }

    fn sum_of_good_subsequences(&self, nums: Vec<i32>) -> i32 {
        let mut cnt: HashMap<i32, i32> = HashMap::new();
        let mut sum: HashMap<i32, i32> = HashMap::new();

        for &i in &nums {
            let cnt_i_minus_1 = *cnt.get(&(i - 1)).unwrap_or(&0);
            let cnt_i_plus_1 = *cnt.get(&(i + 1)).unwrap_or(&0);
            let sum_i_minus_1 = *sum.get(&(i - 1)).unwrap_or(&0);
            let sum_i_plus_1 = *sum.get(&(i + 1)).unwrap_or(&0);

            // Update cnt[i]
            let cnt_i = (cnt_i_minus_1 + cnt_i_plus_1 + 1) % self.mod_val;
            cnt.insert(i, cnt_i);

            // Update sum[i]
            let mut sum_i = (sum_i_minus_1 + sum_i_plus_1) % self.mod_val;
            sum_i += ((cnt_i_minus_1 + cnt_i_plus_1 + 1) as i64 * i as i64 % self.mod_val as i64) as i32;
            sum_i %= self.mod_val;
            sum.insert(i, sum_i);
        }

        // Calculate the final result
        let mut res = 0;
        for &value in sum.values() {
            res = (res + value) % self.mod_val;
        }

        res
    }
}

fn main() {
    // Reading input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // First line contains the size of the array
    let n: usize = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .expect("Failed to parse the size of the array");

    // Second line contains the array elements
    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to parse array element"))
        .collect();

    // Create an object of Solution
    let solution = Solution::new();

    // Call the method and get the result
    let result = solution.sum_of_good_subsequences(nums);

    // Output the result
    println!("{}", result);
}