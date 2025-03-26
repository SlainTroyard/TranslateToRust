use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn sum_of_good_subsequences(nums: &[i32]) -> i32 {
        let mod_val = 1_000_000_007;
        let mut cnt: HashMap<i32, i32> = HashMap::new();
        let mut sum: HashMap<i32, i32> = HashMap::new();

        for &num in nums {
            // Update cnt[i]
            let prev_cnt_minus = *cnt.get(&(num - 1)).unwrap_or(&0);
            let prev_cnt_plus = *cnt.get(&(num + 1)).unwrap_or(&0);
            let delta_cnt = (prev_cnt_minus + prev_cnt_plus + 1) % mod_val;
            let current_cnt = *cnt.get(&num).unwrap_or(&0);
            let new_cnt = (current_cnt as i64 + delta_cnt as i64) % mod_val as i64;
            cnt.insert(num, new_cnt as i32);

            // Update sum[i]
            let prev_sum_minus = *sum.get(&(num - 1)).unwrap_or(&0);
            let prev_sum_plus = *sum.get(&(num + 1)).unwrap_or(&0);
            let delta_sum1 = (prev_sum_minus + prev_sum_plus) % mod_val;
            let current_sum = *sum.get(&num).unwrap_or(&0);
            let mut new_sum = (current_sum as i64 + delta_sum1 as i64) % mod_val as i64;

            // Compute contribution from cnt terms
            let cnt_minus = *cnt.get(&(num - 1)).unwrap_or(&0);
            let cnt_plus = *cnt.get(&(num + 1)).unwrap_or(&0);
            let cnt_terms = (cnt_minus + cnt_plus + 1) % mod_val;
            let delta_sum2 = (cnt_terms as i64 * num as i64) % mod_val as i64;
            new_sum = (new_sum + delta_sum2) % mod_val as i64;

            sum.insert(num, new_sum as i32);
        }

        // Sum all values in 'sum' and apply mod
        let mut res = 0;
        for &val in sum.values() {
            res = (res + val) % mod_val;
        }
        res
    }
}

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());
    let n = iter.next().unwrap() as usize;
    let nums: Vec<i32> = iter.take(n).collect();
    let solution = Solution;
    let result = solution.sum_of_good_subsequences(&nums);
    println!("{}", result);
}