use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn subarray_sum(nums: &Vec<i32>) -> i32 {
        let n = nums.len();
        let mut s = vec![0; n + 1];
        for i in 0..n {
            s[i + 1] = s[i] + nums[i];
        }

        let mut ans = 0;
        for i in 0..n {
            let start = i.checked_sub(nums[i] as usize).unwrap_or(0);
            ans += s[i + 1] - s[start];
        }
        ans
    }
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
    let result = Solution::subarray_sum(&nums);
    println!("{}", result);

    Ok(())
}