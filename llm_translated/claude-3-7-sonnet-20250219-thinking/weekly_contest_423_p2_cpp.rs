use std::io;
use std::cmp;

struct Solution;

impl Solution {
    fn max_increasing_subarrays(nums: &[i32]) -> i32 {
        let mut prev = 0;
        let mut curr = 1;
        let mut ans = 0;

        // Traverse through the nums array
        for i in 1..nums.len() {
            if nums[i - 1] < nums[i] {
                curr += 1;  // Increase the length of the current increasing subarray
            } else {
                prev = curr;  // Update the previous subarray length
                curr = 1;     // Reset current subarray length
            }
            // Update the answer by considering both the current and previous subarray lengths
            ans = cmp::max(ans, cmp::max(curr / 2, cmp::min(prev, curr)));
        }
        ans  // Return the maximum length of increasing subarrays
    }
}

fn main() {
    // Read the size of the array
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n: usize = input.trim().parse().expect("Expected a valid integer");
    
    // Read the array elements
    let mut nums = Vec::with_capacity(n);
    let mut count = 0;
    
    while count < n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        
        for num in input.trim().split_whitespace() {
            if count < n {
                nums.push(num.parse::<i32>().expect("Expected a valid integer"));
                count += 1;
            } else {
                break;
            }
        }
    }
    
    // Create a Solution object and call the function to get the result
    let result = Solution::max_increasing_subarrays(&nums);
    
    // Output the result
    println!("{}", result);
}