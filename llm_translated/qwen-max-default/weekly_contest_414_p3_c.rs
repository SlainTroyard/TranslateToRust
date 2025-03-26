// Problem: Weekly Contest 414 Problem 3

use std::io;

fn find_maximum_score(nums: &[i32]) -> i64 {
    let mut ans = 0;
    let mut l = 0;
    for (r, &num) in nums.iter().enumerate() {
        if nums[l] < num {
            ans += (r - l) as i64 * nums[l] as i64;
            l = r;
        }
    }
    ans + (nums.len() - l - 1) as i64 * nums[l] as i64
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the size of the array
    let mut nums_size = String::new();
    io::stdin().read_line(&mut nums_size)?;
    let nums_size: usize = nums_size.trim().parse()?;

    // Allocate and read the array
    let mut nums: Vec<i32> = Vec::with_capacity(nums_size);
    for _ in 0..nums_size {
        let mut num_str = String::new();
        io::stdin().read_line(&mut num_str)?;
        let num: i32 = num_str.trim().parse()?;
        nums.push(num);
    }

    // Calculate and print the result
    let result = find_maximum_score(&nums);
    println!("{}", result);

    Ok(())
}