use std::io;
use std::io::Read;

fn count_partitions(nums: &[i32]) -> i32 {
    let mut count = 0;
    for i in 0..nums.len() - 1 {
        let left_sum = nums[0..=i].iter().sum::<i32>();
        let right_sum = nums[i+1..].iter().sum::<i32>();
        if (left_sum - right_sum) % 2 == 0 {
            count += 1;
        }
    }
    count
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut tokens = input.split_whitespace();
    let n = tokens.next().and_then(|s| s.parse::<i32>().ok()).expect("No n provided");
    let nums: Vec<i32> = tokens
        .take(n as usize)
        .map(|s| s.parse::<i32>().expect("Invalid number"))
        .collect();

    let count = count_partitions(&nums);
    println!("{}", count);
    Ok(())
}