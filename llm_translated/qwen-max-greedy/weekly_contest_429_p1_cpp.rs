use std::io;
use std::collections::HashSet;

struct Solution;

impl Solution {
    /// Finds the minimum number of operations needed to make all elements in `nums` unique.
    /// An operation consists of removing exactly 3 elements from the end of the array.
    fn minimum_operations(nums: &Vec<i32>) -> i32 {
        let mut seen = HashSet::new();
        for (i, &num) in nums.iter().enumerate().rev() {
            if !seen.insert(num) { // num is already in seen
                return (i as f32 / 3.0).ceil() as i32 + 1;
            }
        }
        0
    }
}

fn main() {
    let mut input = String::new();
    
    // Read the size of the array
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please type a number!");
    
    // Initialize the vector with the given size
    let mut nums = vec![0; n];
    
    // Read the elements of the array
    for num in nums.iter_mut() {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        *num = input.trim().parse().expect("Please type a number!");
    }

    // Create an instance of Solution and call the function
    let solution = Solution;
    let result = solution.minimum_operations(&nums);
    
    // Output the result
    println!("{}", result);
}