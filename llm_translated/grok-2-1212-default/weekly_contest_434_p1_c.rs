use std::io::{self, BufRead};

fn count_partitions(nums: &[i32]) -> i32 {
    let mut count = 0;
    for i in 0..nums.len() - 1 {
        let mut left_sum = 0;
        let mut right_sum = 0;
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
    let result = count_partitions(&nums);
    println!("{}", result);

    Ok(())
}