use std::collections::HashMap;
use std::io::{self, BufRead};

const MOD: i64 = 1_000_000_007;

struct Solution;

impl Solution {
    fn sum_of_good_subsequences(nums: Vec<i32>) -> i32 {
        let mut cnt: HashMap<i32, i64> = HashMap::new();
        let mut sum: HashMap<i32, i64> = HashMap::new();

        for &i in &nums {
            // Update cnt[i] by considering subsequences from i-1, i, and i+1
            let cnt_i_minus_1 = *cnt.get(&(i - 1)).unwrap_or(&0);
            let cnt_i_plus_1 = *cnt.get(&(i + 1)).unwrap_or(&0);
            let cnt_i = (cnt_i_minus_1 + cnt_i_plus_1 + 1) % MOD;
            cnt.insert(i, cnt_i);

            // Update sum[i] by considering subsequences from i-1, i, and i+1
            let sum_i_minus_1 = *sum.get(&(i - 1)).unwrap_or(&0);
            let sum_i_plus_1 = *sum.get(&(i + 1)).unwrap_or(&0);
            let sum_i = (sum_i_minus_1 + sum_i_plus_1) % MOD;
            sum.insert(i, sum_i);

            // Add the contribution of the subsequences that end at i
            let contribution = (cnt_i_minus_1 + cnt_i_plus_1 + 1) % MOD * i as i64 % MOD;
            *sum.get_mut(&i).unwrap() = (sum_i + contribution) % MOD;
        }

        // Calculate the final result by summing all the values in sum
        let mut res = 0;
        for &s in sum.values() {
            res = (res + s) % MOD;
        }

        res as i32
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first line to get the number of elements
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read the second line to get the array of numbers
    let nums: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Create an object of Solution
    let solution = Solution;

    // Call the method and get the result
    let result = solution.sum_of_good_subsequences(nums);

    // Output the result
    println!("{}", result);
}