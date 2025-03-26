use std::io;
use std::collections::HashSet;

struct Solution;

impl Solution {
    /// Finds the minimum number of operations to make all elements in `nums` unique.
    /// The operation is defined as removing the last three elements from `nums`.
    ///
    /// # Arguments
    /// * `nums` - A vector of integers.
    ///
    /// # Returns
    /// * The minimum number of operations required.
    fn minimum_operations(nums: &Vec<i32>) -> i32 {
        let mut seen = HashSet::new();
        for (i, &num) in nums.iter().enumerate().rev() {
            if !seen.insert(num) { // num already in seen
                return (i / 3 + 1) as i32;
            }
        }
        0
    }
}

fn main() {
    let mut solution = Solution;

    // Read the size of the array
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: usize = n.trim().parse().expect("Please type a number!");

    // Read the elements of the array
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let num: i32 = input.trim().parse().expect("Please type a number!");
        nums.push(num);
    }

    // Call the function and output the result
    let result = solution.minimum_operations(&nums);
    println!("{}", result);
}