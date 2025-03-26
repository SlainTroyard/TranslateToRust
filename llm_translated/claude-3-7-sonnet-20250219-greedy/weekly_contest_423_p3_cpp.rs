use std::collections::BTreeMap;
use std::io::{self, BufRead};

struct Solution {
    mod_val: i32,
}

impl Solution {
    fn new() -> Self {
        Solution { mod_val: 1_000_000_007 }
    }

    fn sum_of_good_subsequences(&self, nums: &Vec<i32>) -> i32 {
        let mut cnt: BTreeMap<i32, i32> = BTreeMap::new();
        let mut sum: BTreeMap<i32, i32> = BTreeMap::new();
        
        // Loop through all numbers in the nums array
        for &i in nums {
            // Update cnt[i] by considering subsequences from i-1, i, and i+1
            let cnt_i_minus_1 = *cnt.get(&(i - 1)).unwrap_or(&0);
            let cnt_i_plus_1 = *cnt.get(&(i + 1)).unwrap_or(&0);
            
            let new_cnt_i = (cnt_i_minus_1 + cnt_i_plus_1 + 1) % self.mod_val;
            *cnt.entry(i).or_insert(0) = (cnt.get(&i).unwrap_or(&0) + new_cnt_i) % self.mod_val;
            
            // Update sum[i] by considering subsequences from i-1, i, and i+1
            let sum_i_minus_1 = *sum.get(&(i - 1)).unwrap_or(&0);
            let sum_i_plus_1 = *sum.get(&(i + 1)).unwrap_or(&0);
            
            *sum.entry(i).or_insert(0) = (sum.get(&i).unwrap_or(&0) + (sum_i_minus_1 + sum_i_plus_1) % self.mod_val) % self.mod_val;
            
            // Add the contribution of the subsequences that end at i
            let contribution = ((cnt_i_minus_1 as i64 + cnt_i_plus_1 as i64 + 1) % self.mod_val as i64 * i as i64) % self.mod_val as i64;
            *sum.entry(i).or_insert(0) = (sum.get(&i).unwrap_or(&0) as i64 + contribution) as i32 % self.mod_val;
        }
        
        // Calculate the final result by summing all the values in sum
        let mut res = 0;
        for (_, &value) in &sum {
            res = (res + value) % self.mod_val;
        }
        
        res
    }
}

fn main() {
    // Reading input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Parse the number of elements
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Parse the array elements
    let nums: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Create an object of Solution
    let solution = Solution::new();
    
    // Call the method and get the result
    let result = solution.sum_of_good_subsequences(&nums);
    
    // Output the result
    println!("{}", result);
}