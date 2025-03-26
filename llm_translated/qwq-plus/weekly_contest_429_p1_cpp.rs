use std::io;

/// Solution struct as per the problem's requirements.
pub struct Solution;

impl Solution {
    /// Computes the minimum number of operations by checking elements from the end.
    ///
    /// Iterates backwards through the array, tracking seen elements with a hash set.
    /// When a duplicate is found, returns (current index / 3) + 1 as the result.
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let mut seen = HashSet::new();
        // Iterate from the end of the array backwards
        for (i, &num) in nums.iter().enumerate().rev() {
            if !seen.insert(num) {
                // Found a duplicate, return (i / 3) + 1
                return (i / 3 + 1) as i32;
            }
        }
        0
    }
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input"); // Panic on I/O error as per problem's input constraints

    let tokens: Vec<&str> = input.split_whitespace().collect();
    let n: usize = tokens[0]
        .parse()
        .expect("First token must be a valid integer for array size");
    let nums: Vec<i32> = tokens[1..1 + n]
        .iter()
        .map(|&s| s.parse().expect("Invalid number in array elements"))
        .collect();

    let result = Solution::minimum_operations(nums);
    println!("{}", result);
}