use std::collections::HashMap;
use std::io;

struct Solution {}

impl Solution {
    fn get_largest_outlier(nums: Vec<i32>) -> i32 {
        let mut count_map = HashMap::new();
        let mut sum = 0;

        // Count occurrences and calculate sum
        for num in &nums {
            *count_map.entry(num).or_insert(0) += 1;
            sum += num;
        }

        // Extract unique elements and sort in descending order
        let mut candidates: Vec<i32> = count_map.keys().copied().collect();
        candidates.sort_by(|a, b| b.cmp(a));

        // Check each candidate
        for &n in &candidates {
            let remainder = sum - n;
            if remainder % 2 != 0 {
                continue;
            }
            let d = remainder / 2;
            if count_map.contains_key(&d) {
                if d != n || count_map[&d] > 1 {
                    return n;
                }
            }
        }

        -1
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n: usize = input.trim().parse().expect("Invalid input");

    let mut nums = Vec::with_capacity(n);
    input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let parts: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid integer"))
        .collect();
    nums.extend_from_slice(&parts[..n]);

    let solution = Solution {};
    let result = solution.get_largest_outlier(nums);
    println!("{}", result);
}