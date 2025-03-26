use std::collections::BTreeMap;
use std::io::{self, BufRead};

struct Solution {
    mod_value: i32,
}

impl Solution {
    fn new() -> Self {
        Solution { mod_value: 1_000_000_007 }
    }

    fn sum_of_good_subsequences(&self, nums: &Vec<i32>) -> i32 {
        let mut cnt: BTreeMap<i32, i32> = BTreeMap::new();
        let mut sum: BTreeMap<i32, i32> = BTreeMap::new();
        
        // Loop through all numbers in the nums array
        for &i in nums {
            // Update cnt[i] by considering subsequences from i-1, i, and i+1
            let cnt_i_minus_1 = *cnt.get(&(i - 1)).unwrap_or(&0);
            let cnt_i_plus_1 = *cnt.get(&(i + 1)).unwrap_or(&0);
            
            let entry = cnt.entry(i).or_insert(0);
            *entry = (*entry + cnt_i_minus_1 + cnt_i_plus_1 + 1) % self.mod_value;
            
            // Update sum[i] by considering subsequences from i-1, i, and i+1
            let sum_i_minus_1 = *sum.get(&(i - 1)).unwrap_or(&0);
            let sum_i_plus_1 = *sum.get(&(i + 1)).unwrap_or(&0);
            
            let entry = sum.entry(i).or_insert(0);
            *entry = (*entry + sum_i_minus_1 + sum_i_plus_1) % self.mod_value;
            
            // Add the contribution of the subsequences that end at i
            let contribution = ((cnt_i_minus_1 as i64 + cnt_i_plus_1 as i64 + 1) % self.mod_value as i64 * i as i64) % self.mod_value as i64;
            *entry = (*entry as i64 + contribution) as i32 % self.mod_value;
        }
        
        // Calculate the final result by summing all the values in sum
        let mut res = 0;
        for (_, &val) in sum.iter() {
            res = (res + val) % self.mod_value;
        }
        
        res
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Reading input
    let n: usize = lines.next().unwrap()?.trim().parse().expect("Failed to parse n");
    let nums: Vec<i32> = lines.next().unwrap()?.trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse number"))
        .collect();
    
    // Create an object of Solution
    let solution = Solution::new();
    
    // Call the method and get the result
    let result = solution.sum_of_good_subsequences(&nums);
    
    // Output the result
    println!("{}", result);
    
    Ok(())
}