use std::collections::BTreeMap;
use std::io::{self, BufRead, Write};
use std::error::Error;

// This function implements the logic from the C++ code.
// It finds the maximum subarray sum with the property described in the problem statement.
fn max_subarray_sum(nums: &[i32], k: i32) -> i64 {
    // BTreeMap is used similarly to std::map in C++
    let mut m: BTreeMap<i32, i64> = BTreeMap::new();
    let mut ans = i64::MIN;
    let mut sm: i64 = 0;
    
    // Iterate over the vector while maintaining a running sum
    for (i, &num) in nums.iter().enumerate() {
        sm += num as i64;
        let cur_sz = (i + 1) as i32;
        
        // If the current size is divisible by k, update ans
        if cur_sz % k == 0 {
            ans = ans.max(sm);
        }
        
        let y = cur_sz % k;
        // Check if we have already seen a prefix sum with this remainder
        if let Some(&prev_sm) = m.get(&y) {
            ans = ans.max(sm - prev_sm);
            // Update the stored prefix sum to the minimum (to maximize the difference later)
            m.insert(y, prev_sm.min(sm));
        } else {
            // If not found, simply insert the current sum for this remainder
            m.insert(y, sm);
        }
    }
    ans
}

fn main() -> Result<(), Box<dyn Error>> {
    // Lock stdin to read input efficiently.
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    // Collect all input tokens because the input format may contain multiple lines or multiple values per line.
    let mut input_str = String::new();
    for line in iterator {
        input_str.push_str(&line?);
        input_str.push(' ');
    }
    let tokens: Vec<&str> = input_str.split_whitespace().collect();
    let mut token_iter = tokens.iter();

    // Parse the first two integers: n and k.
    // n: size of the array, and k: the divisor value.
    let n: usize = token_iter.next().ok_or("Expected n")?.parse()?;
    let k: i32 = token_iter.next().ok_or("Expected k")?.parse()?;

    // Parse the array of integers.
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        let num: i32 = token_iter.next().ok_or("Expected a number")?.parse()?;
        nums.push(num);
    }

    // Call the function with the parsed input.
    let result = max_subarray_sum(&nums, k);
    // Print the result in the same format as the original C++ code.
    println!("{}", result);

    Ok(())
}