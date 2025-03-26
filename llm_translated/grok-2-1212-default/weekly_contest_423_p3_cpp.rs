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
            let cnt_prev = *cnt.get(&(num - 1)).unwrap_or(&0);
            let cnt_next = *cnt.get(&(num + 1)).unwrap_or(&0);
            let sum_prev = *sum.get(&(num - 1)).unwrap_or(&0);
            let sum_next = *sum.get(&(num + 1)).unwrap_or(&0);

            // Update cnt[num] by considering subsequences from num-1, num, and num+1
            let new_cnt = (cnt_prev + cnt_next + 1) % self.mod_val;
            *cnt.entry(num).or_insert(0) += new_cnt;
            *cnt.get_mut(&num).unwrap() %= self.mod_val;

            // Update sum[num] by considering subsequences from num-1, num, and num+1
            let new_sum = (sum_prev + sum_next) % self.mod_val;
            *sum.entry(num).or_insert(0) += new_sum;
            *sum.get_mut(&num).unwrap() %= self.mod_val;

            // Add the contribution of the subsequences that end at num
            let contribution = ((cnt_prev + cnt_next + 1) % self.mod_val * num % self.mod_val) % self.mod_val;
            *sum.get_mut(&num).unwrap() += contribution;
            *sum.get_mut(&num).unwrap() %= self.mod_val;
        }

        // Calculate the final result by summing all the values in sum
        let res: i64 = sum.values().sum::<i64>() % self.mod_val;
        res as i32
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Reading input
    let n: usize = lines.next().unwrap()?.trim().parse().unwrap();
    let nums: Vec<i32> = lines.next().unwrap()?.split_whitespace()
        .map(|s| s.trim().parse().unwrap())
        .collect();

    // Create an object of Solution
    let solution = Solution::new();

    // Call the method and get the result
    let result = solution.sum_of_good_subsequences(nums);

    // Output the result
    println!("{}", result);

    Ok(())
}