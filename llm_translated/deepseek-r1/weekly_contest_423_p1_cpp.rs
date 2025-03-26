struct Solution;

impl Solution {
    fn has_increasing_subarrays(nums: &[i32], k: i32) -> bool {
        let k_usize = k as usize;
        let required_length = 2 * k_usize;

        // Early return if the array isn't long enough for two subarrays
        if nums.len() < required_length {
            return false;
        }

        // Define a closure to check if a subarray starting at `idx` is strictly increasing
        let mono = |idx: usize| -> bool {
            // Check each consecutive pair in the subarray of length `k`
            for i in idx..(idx + k_usize - 1) {
                if nums[i] >= nums[i + 1] {
                    return false;
                }
            }
            true
        };

        // Iterate through all possible starting positions for the two subarrays
        let max_idx = nums.len() - required_length;
        for idx in 0..=max_idx {
            if mono(idx) && mono(idx + k_usize) {
                return true;
            }
        }

        false
    }
}

fn main() {
    // Read all input and split into tokens
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");
    let mut tokens = input.split_whitespace().map(|s| s.parse::<i32>().expect("Invalid integer"));

    // Parse input components sequentially
    let n = tokens.next().expect("Missing array size") as usize;
    let nums: Vec<i32> = (0..n)
        .map(|_| tokens.next().expect("Insufficient elements in array"))
        .collect();
    let k = tokens.next().expect("Missing subarray length");

    // Check for increasing subarrays and print result
    let result = Solution::has_increasing_subarrays(&nums, k);
    println!("{}", if result { "true" } else { "false" });
}