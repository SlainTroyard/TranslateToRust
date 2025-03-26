use std::collections::HashMap;
use std::io::{self, BufRead};

// A struct representing the Solution, analogous to the C++ class
struct Solution;

impl Solution {
    // Find x-sum of sliding windows of size k in the nums array
    pub fn find_x_sum(nums: Vec<i32>, k: usize, x: usize) -> Vec<i32> {
        let mut count_map = HashMap::new(); // Maintain count of elements in the current window
        let mut result = Vec::new(); // Result to store the x-sum for each valid window

        let mut l = 0; // Left index of the sliding window
        for (r, &value) in nums.iter().enumerate() {
            // Add the current value to the count map
            *count_map.entry(value).or_insert(0) += 1;

            // Check if the window size equals k
            if r - l + 1 == k {
                // Convert the map to a vector of (num, count) pairs
                let mut vec: Vec<_> = count_map.iter().map(|(&num, &count)| (num, count)).collect();

                // Sort the vector based on the problem conditions
                vec.sort_by(|&(num1, count1), &(num2, count2)| {
                    if count1 == count2 {
                        num2.cmp(&num1) // Larger num comes first if counts are equal
                    } else {
                        count2.cmp(&count1) // Larger count comes first otherwise
                    }
                });

                // Calculate the sum of the top x elements (or less if window has fewer elements)
                let sum: i32 = vec.iter()
                    .take(x)
                    .map(|&(num, count)| num * count)
                    .sum();
                result.push(sum);

                // Remove the leftmost element of the window from the count map
                let left_value = nums[l];
                if let Some(count) = count_map.get_mut(&left_value) {
                    *count -= 1;
                    if *count == 0 {
                        count_map.remove(&left_value);
                    }
                }
                l += 1; // Slide the window to the right
            }
        }

        result
    }
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read k and x
    let first_line = lines.next().unwrap().unwrap();
    let mut first_iter = first_line.split_whitespace();
    let k: usize = first_iter.next().unwrap().parse().unwrap();
    let x: usize = first_iter.next().unwrap().parse().unwrap();

    // Read the size of nums array
    let second_line = lines.next().unwrap().unwrap();
    let nums_size: usize = second_line.parse().unwrap();

    // Read the nums array
    let third_line = lines.next().unwrap().unwrap();
    let nums: Vec<i32> = third_line
        .split_whitespace()
        .take(nums_size)
        .map(|s| s.parse().unwrap())
        .collect();

    // Call the solution method
    let result = Solution::find_x_sum(nums, k, x);

    // Print the result in the required output format
    let output: String = result
        .into_iter()
        .map(|num| num.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    println!("{}", output);
}