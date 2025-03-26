use std::io::{self, BufRead};

fn minimum_operations(nums: &[i32]) -> i32 {
    let mut count = [0; 101]; // Initialize a count array with 101 zeros
    for (i, &num) in nums.iter().enumerate().rev() {
        count[num as usize] += 1;
        if count[num as usize] > 1 {
            return ((i + 3) / 3) as i32;
        }
    }
    0
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first line which contains the number of elements
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read the second line which contains the array elements
    let nums_line = lines.next().unwrap().unwrap();
    let nums: Vec<i32> = nums_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Ensure the number of elements matches the input size
    assert_eq!(nums.len(), n);

    // Compute the result and print it
    let result = minimum_operations(&nums);
    println!("{}", result);
}