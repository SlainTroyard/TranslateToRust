use std::io::{self, BufRead};

fn minimum_operations(nums: &[i32]) -> i32 {
    let mut count = [0u8; 101]; // Count array to track occurrences
    
    // Iterate from the end to the beginning, preserving original index logic
    for i in (0..nums.len()).rev() {
        let num = nums[i] as usize; // Cast to usize for indexing
        count[num] += 1;
        
        // If duplicate found, calculate result using original formula
        if count[num] > 1 {
            return (i as i32 + 3) / 3;
        }
    }
    
    0 // No duplicates found
}

fn main() {
    let stdin = io::stdin();
    let mut tokens = stdin.lock()
        .lines()
        .flat_map(|line| line.unwrap().split_whitespace().map(String::from).collect::<Vec<_>>())
        .into_iter();
    
    // Read array size
    let n = tokens.next()
        .expect("Missing input")
        .parse::<usize>()
        .expect("Invalid array size");
    
    // Read exactly n integers
    let nums: Vec<i32> = tokens.by_ref()
        .take(n)
        .map(|s| s.parse().expect("Invalid integer"))
        .collect();
    
    // Validate input length as per problem constraints
    assert_eq!(nums.len(), n, "Incorrect number of elements");
    
    let result = minimum_operations(&nums);
    println!("{}", result);
}