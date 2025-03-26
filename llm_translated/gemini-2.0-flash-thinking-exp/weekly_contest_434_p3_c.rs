use std::cmp::max;
use std::io;
use std::io::Read;

fn max_frequency(nums: &[i32], k: i32) -> i32 {
    let mut f0 = 0;
    let mut f1 = [0; 51];
    let mut max_f1 = 0;
    let mut f2 = 0;

    for &x in nums {
        // Update f2 - consider current element
        f2 = max(f2, max_f1) + if x == k { 1 } else { 0 };

        // Update the frequency of the current number
        f1[x as usize] = max(f1[x as usize], f0) + 1;

        // If the current element equals k, update f0
        f0 += if x == k { 1 } else { 0 };

        // Update max frequency
        max_f1 = max(max_f1, f1[x as usize]);
    }

    // Return the final result
    max(max_f1, f2)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let first_line = lines.next().ok_or("Missing first line")?;
    let mut parts = first_line.split_whitespace();
    let n: i32 = parts.next().ok_or("Missing n")?.parse()?;
    let k: i32 = parts.next().ok_or("Missing k")?.parse()?;

    let second_line = lines.next().ok_or("Missing second line")?;
    let nums: Vec<i32> = second_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    if nums.len() != n as usize {
        return Err("Incorrect number of elements in nums".into());
    }

    let result = max_frequency(&nums, k);

    println!("{}", result);

    Ok(())
}