use std::io::{self, Write};

fn find_maximum_score(nums: &[i32]) -> i64 {
    let mut ans = 0_i64;
    let mut l = 0_usize;
    let mut r = 1_usize;

    while r < nums.len() {
        if nums[l] < nums[r] {
            ans += (r - l) as i64 * nums[l] as i64;
            l = r;
        }
        r += 1;
    }

    ans + (r - l - 1) as i64 * nums[l] as i64
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read `numsSize` from input
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let nums_size: usize = input.trim().parse()?;

    // Read the array `nums` from input
    input.clear();
    io::stdin().read_line(&mut input)?;
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|num| num.parse::<i32>())
        .collect::<Result<Vec<i32>, _>>()?;

    // Ensure the input size matches the expected `numsSize`
    if nums.len() != nums_size {
        return Err("The size of the input array doesn't match numsSize.".into());
    }

    // Compute the result
    let result = find_maximum_score(&nums);

    // Print the result
    writeln!(io::stdout(), "{}", result)?;

    Ok(())
}