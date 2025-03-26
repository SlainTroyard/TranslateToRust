use std::io;
use std::cmp::max;

struct Solution {}

impl Solution {
    pub fn maximum_coins(nums: Vec<Vec<i32>>, k: i32) -> i64 {
        let mut nums = nums;
        nums.sort_by(|a, b| a[0].cmp(&b[0]));

        let n = nums.len();
        let mut presum: Vec<i64> = vec![0; n + 1];
        for i in 1..=n {
            presum[i] = presum[i - 1] + ((nums[i - 1][1] as i64 - nums[i - 1][0] as i64 + 1) * nums[i - 1][2] as i64);
        }

        let mut ans: i64 = 0;
        let mut left: usize = 0;
        let mut right: usize = 0;

        while right < n && left < n {
            while left < n && nums[right][1] as i64 - nums[left][0] as i64 + 1 > k as i64 {
                let tamp = k as i64 - (nums[right][0] as i64 - nums[left][0] as i64);
                ans = max(ans, tamp * nums[right][2] as i64 + presum[right] - presum[left]);
                left += 1;
            }
            if left > n {
                break;
            }
            ans = max(ans, presum[right + 1] - presum[left]);
            right += 1;
        }

        left = n - 1;
        right = n - 1;

        while right >= 0 && left >= 0 {
            while right >= 0 && nums[right][1] as i64 - nums[left][0] as i64 + 1 > k as i64 {
                let tamp = k as i64 - (nums[right][1] as i64 - nums[left][1] as i64);
                ans = max(ans, tamp * nums[left][2] as i64 + presum[right + 1] - presum[left + 1]);
                if right == 0 {
                    break;
                }
                right -= 1;
            }
            if right < 0 {
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

fn main() {
    let mut n_k = String::new();
    io::stdin().read_line(&mut n_k).expect("Failed to read line");
    let mut iter = n_k.trim().split_whitespace();
    let n: i32 = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();

    let mut vec: Vec<Vec<i32>> = Vec::new();
    for _i in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read line");
        let nums: Vec<i32> = line
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        vec.push(nums);
    }

    let sol = Solution {};
    println!("{}", sol.maximum_coins(vec, k));
}