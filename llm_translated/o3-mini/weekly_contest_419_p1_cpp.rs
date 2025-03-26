use std::collections::HashMap;
use std::io::{self, BufRead};

/// This function calculates the x-sum for every contiguous subarray of length k
/// from the given vector `nums`. In each window, it computes the frequency of each number,
/// then sorts the (number, count) pairs with the frequency in descending order,
/// and if frequencies are equal, by the number value descending. Then it sums up the first x
/// pairs where each pair contributes (number * count).
fn find_x_sum(nums: &[i32], k: usize, x: usize) -> Vec<i32> {
    let mut res = Vec::new();
    let mut freq: HashMap<i32, i32> = HashMap::new();
    let mut l = 0;
    // r is the right index of the window
    for r in 0..nums.len() {
        // Increase the count for nums[r]
        *freq.entry(nums[r]).or_insert(0) += 1;
        
        // When we've reached a window of size k, process it.
        if r - l + 1 == k {
            // Copy the map entries into a vector of (num, count) pairs
            let mut vec: Vec<(i32, i32)> = freq.iter().map(|(&num, &count)| (num, count)).collect();
            // Sort by descending frequency; if frequency equal, sort by number descending.
            vec.sort_unstable_by(|a, b| {
                // Compare counts
                match b.1.cmp(&a.1) {
                    std::cmp::Ordering::Equal => b.0.cmp(&a.0),
                    ord => ord,
                }
            });
            
            // Sum up the top x pairs, if available.
            let mut sum = 0;
            for i in 0..x.min(vec.len()) {
                sum += vec[i].0 * vec[i].1;
            }
            res.push(sum);
            
            // Slide the window: remove the leftmost element from the frequency map.
            if let Some(count) = freq.get_mut(&nums[l]) {
                *count -= 1;
                if *count == 0 {
                    freq.remove(&nums[l]);
                }
            }
            l += 1;
        }
    }
    res
}

/// Helper function to read all input from stdin and tokenize it by whitespace.
fn read_tokens() -> Vec<String> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut tokens = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        // Split the line by whitespace and collect all tokens.
        tokens.extend(line.split_whitespace().map(String::from));
    }
    tokens
}

fn main() {
    // Read all tokens from stdin.
    let tokens = read_tokens();
    let mut iter = tokens.into_iter();
    
    // Parse k and x from the input.
    let k: usize = iter
        .next()
        .expect("Expected value for k")
        .parse()
        .expect("k should be a number");
    let x: usize = iter
        .next()
        .expect("Expected value for x")
        .parse()
        .expect("x should be a number");
    
    // Parse the size of the vector.
    let nums_size: usize = iter
        .next()
        .expect("Expected value for numsSize")
        .parse()
        .expect("numsSize should be a number");
    
    // Parse the nums vector.
    let mut nums = Vec::with_capacity(nums_size);
    for _ in 0..nums_size {
        let num = iter
            .next()
            .expect("Expected a number in nums")
            .parse()
            .expect("Each num should be an integer");
        nums.push(num);
    }
    
    // Get the result by computing find_x_sum.
    let result = find_x_sum(&nums, k, x);
    
    // Print the results separated by space, ending with a newline.
    // The output format matches the original C++ code.
    let output: String = result
        .iter()
        .map(|num| num.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    println!("{}", output);
}