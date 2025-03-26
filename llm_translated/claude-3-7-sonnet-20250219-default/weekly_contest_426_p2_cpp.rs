use std::collections::HashMap;
use std::io::{self, BufRead};

struct Solution {}

impl Solution {
    pub fn get_largest_outlier(nums: Vec<i32>) -> i32 {
        let mut counter: HashMap<i32, i32> = HashMap::new();
        let mut sum = 0;

        // Count occurrences of each number and calculate sum
        for num in &nums {
            *counter.entry(*num).or_insert(0) += 1;
            sum += num;
        }

        // Extract unique values and sort in descending order
        let mut candidates: Vec<i32> = counter.keys().cloned().collect();
        candidates.sort_unstable_by(|a, b| b.cmp(a)); // Sort in descending order

        // Check each candidate
        for n in candidates {
            let d = (sum - n) / 2;
            let m = (sum - n) % 2;
            
            // Check if a valid outlier
            if m == 0 && counter.contains_key(&d) && (d != n || counter[&d] > 1) {
                return n;
            }
        }
        
        // No valid outlier found
        -1
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the number of elements
    let n: usize = lines.next().unwrap()?.trim().parse().expect("Invalid input for n");
    
    // Read the elements of the array
    let nums_input = lines.next().unwrap()?;
    let nums: Vec<i32> = nums_input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid integer in nums"))
        .collect();
    
    // Call the method and output the result
    let result = Solution::get_largest_outlier(nums);
    println!("{}", result);
    
    Ok(())
}