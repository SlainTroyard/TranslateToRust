// Problem: Weekly Contest 429 Problem 1
use std::io;

fn minimum_operations(nums: &Vec<i32>) -> i32 {
    let mut count = [0; 101];
    for (i, &num) in nums.iter().enumerate().rev() {
        count[num as usize] += 1;
        if count[num as usize] > 1 {
            return (i + 3) / 3;
        }
    }
    0
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the size of the array from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let n: usize = input.trim().parse()?;

    // Read the elements of the array from stdin
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input)?;
        let num: i32 = input.trim().parse()?;
        nums.push(num);
    }

    // Calculate the result and print it to stdout
    let result = minimum_operations(&nums);
    println!("{}", result);

    Ok(())
}