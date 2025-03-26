use std::io::{self, Write};

fn minimum_operations(nums: &[i32]) -> i32 {
    let mut count = [0; 101]; // Initialize an array to count occurrences of each number
    for (i, &num) in nums.iter().enumerate().rev() {
        count[num as usize] += 1; // Increment the count for the current number
        if count[num as usize] > 1 {
            return ((i + 3) / 3) as i32; // Return the minimum operations if a duplicate is found
        }
    }
    0 // Return 0 if no duplicates are found
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Invalid input for n");

    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let num: i32 = input.trim().parse().expect("Invalid input for num");
        nums.push(num);
    }

    let result = minimum_operations(&nums);
    println!("{}", result);
}