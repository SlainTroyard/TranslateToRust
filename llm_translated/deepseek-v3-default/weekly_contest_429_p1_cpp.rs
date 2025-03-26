use std::collections::HashSet;
use std::io;

fn main() {
    // Create a new instance of the Solution struct
    let solution = Solution;

    // Read the size of the array from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n: usize = input.trim().parse().expect("Invalid input");

    // Read the elements of the array from stdin
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let num: i32 = input.trim().parse().expect("Invalid input");
        nums.push(num);
    }

    // Call the minimum_operations function and output the result
    let result = solution.minimum_operations(nums);
    println!("{}", result);
}

struct Solution;

impl Solution {
    // Function to calculate the minimum number of operations
    fn minimum_operations(&self, nums: Vec<i32>) -> i32 {
        let mut seen = HashSet::new();
        for i in (0..nums.len()).rev() {
            if !seen.insert(nums[i]) {
                return (i / 3 + 1) as i32;
            }
        }
        0
    }
}