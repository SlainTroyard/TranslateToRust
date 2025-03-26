use std::collections::HashMap;
use std::io::{self, BufRead};

struct Solution {
    mod_val: i32,
}

impl Solution {
    fn new() -> Self {
        Solution {
            mod_val: 1_000_000_007,
        }
    }

    fn sum_of_good_subsequences(&self, nums: Vec<i32>) -> i32 {
        let mut cnt: HashMap<i32, i32> = HashMap::new();
        let mut sum: HashMap<i32, i32> = HashMap::new();

        for &i in &nums {
            // Update cnt[i] by considering subsequences from i-1, i, and i+1
            let cnt_left = *cnt.get(&(i - 1)).unwrap_or(&0);
            let cnt_right = *cnt.get(&(i + 1)).unwrap_or(&0);
            *cnt.entry(i).or_insert(0) += (cnt_left + cnt_right + 1) % self.mod_val;
            *cnt.get_mut(&i).unwrap() %= self.mod_val;

            // Update sum[i] by considering subsequences from i-1, i, and i+1
            let sum_left = *sum.get(&(i - 1)).unwrap_or(&0);
            let sum_right = *sum.get(&(i + 1)).unwrap_or(&0);
            *sum.entry(i).or_insert(0) += (sum_left + sum_right) % self.mod_val;
            *sum.get_mut(&i).unwrap() %= self.mod_val;

            // Add the contribution of the subsequences that end at i
            *sum.entry(i).or_insert(0) +=
                ((cnt_left + cnt_right + 1) % self.mod_val) * i % self.mod_val;
            *sum.get_mut(&i).unwrap() %= self.mod_val;
        }

        // Calculate the final result by summing all the values in `sum`
        let mut res = 0;
        for &value in sum.values() {
            res += value;
            res %= self.mod_val;
        }

        res
    }
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // First line: number of elements in the array
    let n: usize = lines
        .next()
        .expect("Failed to read line")
        .expect("Failed to read")
        .trim()
        .parse()
        .expect("Failed to parse n");

    // Second line: the array of integers
    let nums: Vec<i32> = lines
        .next()
        .expect("Failed to read line")
        .expect("Failed to read")
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to parse number"))
        .collect();

    assert_eq!(nums.len(), n, "Input array length does not match n");

    // Create a Solution instance
    let solution = Solution::new();

    // Calculate the result
    let result = solution.sum_of_good_subsequences(nums);

    // Output the result
    println!("{}", result);
}