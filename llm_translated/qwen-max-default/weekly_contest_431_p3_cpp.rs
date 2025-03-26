use std::cmp;
use std::io::{self, BufRead};

fn main() {
    // Read the input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Parse n and K
    let first_line = lines.next().unwrap().expect("Failed to read line");
    let mut iter = first_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().expect("Failed to parse n");
    let k: usize = iter.next().unwrap().parse().expect("Failed to parse k");

    // Parse the vector of vectors
    let mut nums: Vec<Vec<i32>> = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap().expect("Failed to read line");
        let mut iter = line.split_whitespace();
        let a: i32 = iter.next().unwrap().parse().expect("Failed to parse a");
        let b: i32 = iter.next().unwrap().parse().expect("Failed to parse b");
        let c: i32 = iter.next().unwrap().parse().expect("Failed to parse c");
        nums.push(vec![a, b, c]);
    }

    // Create a solution instance and compute the result
    let sol = Solution {};
    let result = sol.maximum_coins(nums, k);

    // Print the result
    println!("{}", result);
}

struct Solution;

impl Solution {
    fn maximum_coins(mut nums: Vec<Vec<i32>>, k: usize) -> i64 {
        // Sort the nums vector based on the first element
        nums.sort_by(|a, b| a[0].cmp(&b[0]));

        let mut ans: i64 = 0;
        let mut presum: Vec<i64> = vec![0; nums.len() + 1];

        // Calculate the prefix sum
        for i in 1..=nums.len() {
            presum[i] = presum[i - 1] + (1i64 * (nums[i - 1][1] - nums[i - 1][0] + 1) as i64 * nums[i - 1][2] as i64);
        }

        let mut left = 0;
        let mut right = 0;
        while right < nums.len() && left < nums.len() {
            while left < nums.len() && (nums[right][1] - nums[left][0] + 1) as usize > k {
                let tamp = k as i64 - ((nums[right][0] - nums[left][0]) as i64);
                ans = cmp::max(ans, tamp * nums[right][2] as i64 + presum[right] - presum[left]);
                left += 1;
            }
            if left > nums.len() {
                break;
            }
            ans = cmp::max(ans, presum[right + 1] - presum[left]);
            right += 1;
        }

        left = nums.len() - 1;
        right = nums.len() - 1;
        while right >= 0 && left >= 0 {
            while right >= 0 && (nums[right][1] - nums[left][0] + 1) as usize > k {
                let tamp = k as i64 - ((nums[right][1] - nums[left][1]) as i64);
                ans = cmp::max(ans, tamp * nums[left][2] as i64 + presum[right + 1] - presum[left + 1]);
                right -= 1;
            }
            if right < 0 {
                break;
            }
            ans = cmp::max(ans, presum[right + 1] - presum[left]);
            left -= 1;
        }

        ans
    }
}