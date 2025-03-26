use std::collections::HashSet;
use std::io::{self, Read};

struct Solution;

impl Solution {
    fn minimum_operations(nums: &Vec<i32>) -> i32 {
        let mut seen = HashSet::new();
        for i in (0..nums.len()).rev() {
            // If the element is already in the set, insert will return false
            if !seen.insert(nums[i]) {
                return (i / 3 + 1) as i32;
            }
        }
        0
    }
}

fn main() {
    // Read all input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read from stdin");
    let mut iter = input.split_whitespace();
    
    // Input size of the array
    let n: usize = iter.next()
                       .expect("Missing array size")
                       .parse()
                       .expect("Array size must be a number");
    
    // Input elements of the array
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        let num: i32 = iter.next()
                          .expect("Missing array element")
                          .parse()
                          .expect("Array element must be a number");
        nums.push(num);
    }
    
    // Call the function and output the result
    let result = Solution::minimum_operations(&nums);
    println!("{}", result);
}