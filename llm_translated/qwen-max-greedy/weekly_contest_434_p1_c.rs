// Problem: Weekly Contest 434 Problem 1

use std::io;

fn count_partitions(nums: &Vec<i32>) -> i32 {
    let mut count = 0;
    for i in 0..nums.len() - 1 {
        let (mut left_sum, mut right_sum) = (0, 0);
        for j in 0..=i {
            left_sum += nums[j];
        }
        for j in i + 1..nums.len() {
            right_sum += nums[j];
        }
        if (left_sum - right_sum) % 2 == 0 {
            count += 1;
        }
    }
    count
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the number of elements
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let n: usize = input.trim().parse()?;

    // Read the elements into a vector
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input)?;
        let num: i32 = input.trim().parse()?;
        nums.push(num);
    }

    // Calculate and print the result
    let result = count_partitions(&nums);
    println!("{}", result);

    Ok(())
}