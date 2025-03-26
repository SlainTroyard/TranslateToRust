use std::collections::HashSet;

struct Solution;

impl Solution {
    fn minimum_operations(&self, nums: Vec<i32>) -> i32 {
        let mut seen = HashSet::new();
        for i in (0..nums.len()).rev() {
            let num = nums[i];
            if !seen.insert(num) {
                return (i as i32) / 3 + 1;
            }
        }
        0
    }
}

fn main() {
    use std::io;
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut parts = input.trim().split_whitespace();
    let n: usize = parts.next().expect("Missing n").parse().expect("Invalid n");
    let nums: Vec<i32> = parts.take(n).map(|s| s.parse().expect("Invalid number")).collect();
    
    let solution = Solution;
    let result = solution.minimum_operations(nums);
    println!("{}", result);
}