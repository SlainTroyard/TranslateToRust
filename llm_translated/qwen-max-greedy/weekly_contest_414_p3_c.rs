// Problem: Weekly Contest 414 Problem 3
use std::io;

/// Finds the maximum score based on the given algorithm.
fn find_maximum_score(nums: &[i32]) -> i64 {
    let mut ans = 0;
    let mut l = 0;
    for r in 1..nums.len() {
        if nums[l] < nums[r] {
            ans += (r - l) as i64 * nums[l] as i64;
            l = r;
        }
    }
    ans + (nums.len() - l - 1) as i64 * nums[l] as i64
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the number of elements from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let nums_size: usize = input.trim().parse()?;

    // Allocate a vector to store the numbers
    let mut nums = vec![0; nums_size];

    // Read the numbers from stdin
    for num in nums.iter_mut() {
        input.clear();
        io::stdin().read_line(&mut input)?;
        *num = input.trim().parse()?;
    }

    // Calculate and print the result
    let result = find_maximum_score(&nums);
    println!("{}", result);

    Ok(())
}