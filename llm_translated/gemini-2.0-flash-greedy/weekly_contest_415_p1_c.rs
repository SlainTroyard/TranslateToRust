use std::io;
use std::io::Read;
use std::str::FromStr;

fn get_sneaky_numbers(nums: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(2);
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if nums[i] == nums[j] {
                result.push(nums[i]);
                break;
            }
        }
        if result.len() == 2 {
            break;
        }
    }
    result
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let num_size: usize = lines.next().unwrap().parse::<usize>()? + 2;

    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let result = get_sneaky_numbers(&nums);

    for &num in &result {
        print!("{} ", num);
    }
    println!();

    Ok(())
}