// Problem: Weekly Contest 429 Problem 1

use std::io;

fn minimum_operations(nums: &Vec<i32>) -> i32 {
    let mut count = [0; 101];
    for (i, &num) in nums.iter().enumerate().rev() {
        count[num as usize] += 1;
        if count[num as usize] > 1 {
            return ((nums.len() - i) + 2) as i32 / 3;
        }
    }
    0
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the number of elements
    let mut n = String::new();
    io::stdin().read_line(&mut n)?;
    let n: usize = n.trim().parse()?;

    // Read the elements into a vector
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        let mut num_str = String::new();
        io::stdin().read_line(&mut num_str)?;
        let num: i32 = num_str.trim().parse()?;
        nums.push(num);
    }

    // Calculate the result
    let result = minimum_operations(&nums);

    // Print the result
    println!("{}", result);

    Ok(())
}