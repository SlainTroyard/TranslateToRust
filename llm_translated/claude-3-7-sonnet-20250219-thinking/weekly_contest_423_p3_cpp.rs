use std::collections::BTreeMap;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn sum_of_good_subsequences(nums: &[i32]) -> i32 {
        let mod_val = 1_000_000_007;
        let mut cnt: BTreeMap<i32, i32> = BTreeMap::new();
        let mut sum: BTreeMap<i32, i32> = BTreeMap::new();
        
        // Loop through all numbers in the nums array
        for &i in nums {
            // Get counts from adjacent values
            let cnt_i_minus_1 = *cnt.get(&(i - 1)).unwrap_or(&0);
            let cnt_i_plus_1 = *cnt.get(&(i + 1)).unwrap_or(&0);
            
            // Update cnt[i] by considering subsequences from i-1, i, and i+1
            let cnt_entry = cnt.entry(i).or_insert(0);
            *cnt_entry += (cnt_i_minus_1 + cnt_i_plus_1 + 1) % mod_val;
            *cnt_entry %= mod_val;
            
            // Get sums from adjacent values
            let sum_i_minus_1 = *sum.get(&(i - 1)).unwrap_or(&0);
            let sum_i_plus_1 = *sum.get(&(i + 1)).unwrap_or(&0);
            
            // Update sum[i] by considering subsequences from i-1, i, and i+1
            let sum_entry = sum.entry(i).or_insert(0);
            *sum_entry += (sum_i_minus_1 + sum_i_plus_1) % mod_val;
            *sum_entry %= mod_val;
            
            // Add the contribution of the subsequences that end at i
            // Using i64 for intermediate calculations to prevent overflow
            let contribution = ((cnt_i_minus_1 as i64 + cnt_i_plus_1 as i64 + 1) % mod_val as i64 * i as i64) % mod_val as i64;
            *sum_entry = (*sum_entry as i64 + contribution) as i32 % mod_val;
        }
        
        // Calculate the final result by summing all the values in sum
        let mut res = 0;
        for (_, &value) in &sum {
            res = (res + value) % mod_val;
        }
        
        res
    }
}

fn main() -> io::Result<()> {
    // Setup input reader
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Reading the number of elements
    let n: usize = lines.next().unwrap()?.trim().parse()
        .expect("Failed to parse number of elements");
    
    // Reading the array elements
    let nums: Vec<i32> = lines.next().unwrap()?.trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse array element"))
        .collect();
    
    // Ensure we have the correct number of elements
    assert_eq!(nums.len(), n, "Input array length does not match expected count");
    
    // Call the method and get the result
    let result = Solution::sum_of_good_subsequences(&nums);
    
    // Output the result
    println!("{}", result);
    
    Ok(())
}