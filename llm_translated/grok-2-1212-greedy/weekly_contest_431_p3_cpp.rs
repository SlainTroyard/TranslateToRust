use std::cmp::max;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn maximum_coins(nums: &Vec<Vec<i32>>, k: i32) -> i64 {
        let mut nums = nums.clone();
        nums.sort_by(|a, b| a[0].cmp(&b[0]));

        let mut ans: i64 = 0;
        let mut presum: Vec<i64> = vec![0; nums.len() + 1];
        for i in 1..nums.len() + 1 {
            presum[i] = presum[i - 1] + (nums[i - 1][1] - nums[i - 1][0] + 1) as i64 * nums[i - 1][2] as i64;
        }

        let mut left: usize = 0;
        let mut right: usize = 0;
        while right < nums.len() && left < nums.len() {
            while left < nums.len() && nums[right][1] - nums[left][0] + 1 > k {
                let tamp = k - (nums[right][0] - nums[left][0]);
                ans = max(ans, tamp as i64 * nums[right][2] as i64 + presum[right] - presum[left]);
                left += 1;
            }
            if left >= nums.len() {
                break;
            }
            ans = max(ans, presum[right + 1] - presum[left]);
            right += 1;
        }

        left = nums.len() - 1;
        right = nums.len() - 1;
        while right < nums.len() && left < nums.len() {
            while right < nums.len() && nums[right][1] - nums[left][0] + 1 > k {
                let tamp = k - (nums[right][1] - nums[left][1]);
                ans = max(ans, tamp as i64 * nums[left][2] as i64 + presum[right + 1] - presum[left + 1]);
                if right == 0 {
                    break;
                }
                right -= 1;
            }
            if right >= nums.len() {
                break;
            }
            ans = max(ans, presum[right + 1] - presum[left]);
            if left == 0 {
                break;
            }
            left -= 1;
        }

        ans
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read n and K
    let first_line = lines.next().unwrap()?;
    let mut iter = first_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();

    // Read the vector of vectors
    let mut vec: Vec<Vec<i32>> = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap()?;
        let nums: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        vec.push(nums);
    }

    // Calculate and print the result
    let sol = Solution;
    println!("{}", sol.maximum_coins(&vec, k));

    Ok(())
}