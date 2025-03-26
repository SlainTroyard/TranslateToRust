use std::cmp::max;
use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let first_line = lines.next().unwrap();
    let mut first_line_iter = first_line.split_whitespace();
    let n: usize = first_line_iter.next().unwrap().parse().unwrap();
    let k: i64 = first_line_iter.next().unwrap().parse().unwrap();

    let mut nums: Vec<Vec<i64>> = Vec::new();
    for _ in 0..n {
        let line = lines.next().unwrap();
        let mut iter = line.split_whitespace();
        let a: i64 = iter.next().unwrap().parse().unwrap();
        let b: i64 = iter.next().unwrap().parse().unwrap();
        let c: i64 = iter.next().unwrap().parse().unwrap();
        nums.push(vec![a, b, c]);
    }

    let sol = Solution {};
    println!("{}", sol.maximum_coins(nums, k));
}

struct Solution {}

impl Solution {
    fn maximum_coins(&self, mut nums: Vec<Vec<i64>>, k: i64) -> i64 {
        nums.sort_by(|a, b| a[0].cmp(&b[0]));

        let n = nums.len();
        let mut ans: i64 = 0;
        let mut presum: Vec<i64> = vec![0; n + 1];

        for i in 1..=n {
            presum[i] = presum[i - 1] + (nums[i - 1][1] - nums[i - 1][0] + 1) * nums[i - 1][2];
        }

        let mut left: usize = 0;
        let mut right: usize = 0;

        while right < n && left < n {
            while left < n && nums[right][1] - nums[left][0] + 1 > k {
                let tamp: i64 = k - (nums[right][0] - nums[left][0]);
                ans = max(ans, tamp * nums[right][2] + presum[right] - presum[left]);
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
            while right >= 0 && nums[right][1] - nums[left][0] + 1 > k {
                let tamp: i64 = k - (nums[right][1] - nums[left][1]);
                ans = max(ans, tamp * nums[left][2] + presum[right + 1] - presum[left + 1]);
                if right == 0 {
                    right = usize::MAX;
                    break;
                }
                right -= 1;
            }
            if right == usize::MAX || right < 0 {
                break;
            }
            ans = max(ans, presum[right + 1] - presum[left]);
            if left == 0 {
                left = usize::MAX;
                break;
            }
            left -= 1;
        }

        return ans;
    }
}