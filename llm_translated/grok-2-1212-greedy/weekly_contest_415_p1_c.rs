use std::io::{self, BufRead};

fn get_sneaky_numbers(nums: &[i32]) -> Vec<i32> {
    let mut result = vec![0; 2];
    let mut count = 0;
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            if nums[i] == nums[j] {
                result[count] = nums[i];
                count += 1;
                break;
            }
        }
        if count == 2 {
            break;
        }
    }
    result
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first line for numSize
    let num_size: i32 = lines.next().unwrap()?.trim().parse().unwrap();
    let num_size = (num_size + 2) as usize;

    // Read the next numSize lines for nums
    let mut nums = Vec::with_capacity(num_size);
    for _ in 0..num_size {
        let num: i32 = lines.next().unwrap()?.trim().parse().unwrap();
        nums.push(num);
    }

    // Get sneaky numbers
    let result = get_sneaky_numbers(&nums);

    // Print the result
    for &num in &result {
        print!("{} ", num);
    }
    println!();

    Ok(())
}