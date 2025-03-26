struct Solution;

impl Solution {
    fn count_partitions(nums: &[i32]) -> i32 {
        let s = nums.iter().sum::<i32>();
        if s % 2 != 0 {
            0
        } else {
            (nums.len() - 1) as i32
        }
    }
}

use std::io;

fn main() -> io::Result<()> {
    let mut n = String::new();
    io::stdin().read_line(&mut n)?;
    let n: usize = n.trim().parse()?;

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let nums: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .take(n)
        .collect();

    let sol = Solution;
    println!("{}", sol.count_partitions(&nums));
    Ok(())
}