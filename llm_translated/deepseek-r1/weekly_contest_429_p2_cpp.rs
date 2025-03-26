use std::collections::BTreeSet;

struct Solution;

impl Solution {
    pub fn max_distinct_elements(arr: &mut Vec<i32>, diff: i32) -> usize {
        // Sort the array to process elements in non-decreasing order
        arr.sort();
        let mut prev = i32::MIN;
        let mut set = BTreeSet::new();

        for &num in arr.iter() {
            // Calculate the minimum possible value we can assign for this element
            let x = std::cmp::max(prev + 1, num - diff);
            
            // Check if the calculated value is within the valid range
            if x <= num + diff {
                set.insert(x);
                prev = x;  // Update previous value for next iteration
            }
        }

        set.len()
    }
}

fn main() {
    use std::io::{self, Read};

    // Read all input at once for efficient parsing
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    
    // Split input into whitespace-separated tokens and parse as integers
    let mut tokens = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());
    
    // Parse first two values as n and diff
    let n = tokens.next().expect("Missing array size");
    let diff = tokens.next().expect("Missing difference value");
    
    // Collect next n tokens as the array elements
    let mut arr: Vec<i32> = tokens.take(n as usize).collect();
    
    // Ensure we received exactly n elements as promised
    assert_eq!(arr.len(), n as usize, "Incorrect number of array elements");
    
    // Calculate and print the result
    let result = Solution::max_distinct_elements(&mut arr, diff);
    println!("{}", result);
}