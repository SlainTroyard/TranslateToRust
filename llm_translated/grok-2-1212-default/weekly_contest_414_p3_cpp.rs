use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn find_maximum_score(nums: &Vec<i32>) -> i64 {
        let mut ans: i64 = 0;
        let mut mx: i32 = 0;
        for i in 0..nums.len() - 1 {
            mx = mx.max(nums[i]);
            ans += mx as i64;
        }
        ans
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the nums vector
    let nums_size: usize = lines.next().unwrap()?.trim().parse().unwrap();

    // Read the nums vector
    let nums: Vec<i32> = lines
        .take(nums_size)
        .map(|line| line.unwrap().trim().parse().unwrap())
        .collect();

    // Calculate and print the result
    let result = Solution::find_maximum_score(&nums);
    println!("{}", result);

    Ok(())
}