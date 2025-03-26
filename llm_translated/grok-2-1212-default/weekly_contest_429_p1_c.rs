use std::io::{self, BufRead};

fn minimum_operations(nums: &[i32]) -> i32 {
    let mut count = [0; 101];
    for i in (0..nums.len()).rev() {
        count[nums[i] as usize] += 1;
        if count[nums[i] as usize] > 1 {
            return ((i + 3) / 3) as i32;
        }
    }
    0
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of elements
    let n: usize = lines.next().unwrap()?.trim().parse().unwrap();

    // Read the array elements
    let nums: Vec<i32> = lines
        .take(n)
        .map(|line| line.unwrap().trim().parse().unwrap())
        .collect();

    // Calculate and print the result
    let result = minimum_operations(&nums);
    println!("{}", result);

    Ok(())
}