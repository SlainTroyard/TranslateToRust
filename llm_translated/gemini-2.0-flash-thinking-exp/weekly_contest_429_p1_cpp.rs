use std::io;
use std::collections::HashSet;

fn minimum_operations(nums: &Vec<i32>) -> i32 {
    let mut seen: HashSet<i32> = HashSet::new();
    for i in (0..nums.len()).rev() {
        if !seen.insert(nums[i]) { // If insert returns false, it means nums[i] was already in seen
            return (i as i32) / 3 + 1;
        }
    }
    0
}

fn main() {
    let mut n_str = String::new();
    io::stdin().read_line(&mut n_str).expect("Failed to read line");
    let n: i32 = n_str.trim().parse().expect("Invalid input for n");

    let mut nums_str = String::new();
    io::stdin().read_line(&mut nums_str).expect("Failed to read line");
    let nums: Vec<i32> = nums_str
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input for nums element"))
        .collect();

    let result = minimum_operations(&nums);
    println!("{}", result);
}