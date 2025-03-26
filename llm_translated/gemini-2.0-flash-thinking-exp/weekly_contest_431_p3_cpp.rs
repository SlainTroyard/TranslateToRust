use std::cmp::max;
use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let first_line = lines.next().unwrap();
    let mut split = first_line.split_whitespace();
    let n: usize = split.next().unwrap().parse().unwrap();
    let k: i64 = split.next().unwrap().parse().unwrap();

    let mut vec: Vec<Vec<i32>> = Vec::new();
    for _ in 0..n {
        let line = lines.next().unwrap();
        let mut split = line.split_whitespace();
        let a: i32 = split.next().unwrap().parse().unwrap();
        let b: i32 = split.next().unwrap().parse().unwrap();
        let c: i32 = split.next().unwrap().parse().unwrap();
        vec.push(vec![a, b, c]);
    }

    let sol = Solution {};
    println!("{}", sol.maximum_coins(vec, k));
}

struct Solution {}

impl Solution {
    fn maximum_coins(&self, nums: Vec<Vec<i32>>, k: i64) -> i64 {
        let mut nums = nums; // Make nums mutable
        nums.sort_by(|a, b| a[0].cmp(&b[0]));

        let mut ans: i64 = 0;
        let mut presum: Vec<i64> = vec![0; nums.len() + 1];

        for i in 1..=nums.len() {
            presum[i] = presum[i - 1] + ((nums[i - 1][1] as i64 - nums[i - 1][0] as i64 + 1) * nums[i - 1][2] as i64);
        }

        let mut left: usize = 0;
        let mut right: usize = 0;

        while right < nums.len() && left < nums.len() {
            while left < nums.len() && (nums[right][1] as i64 - nums[left][0] as i64 + 1) > k {
                let tamp: i64 = k - (nums[right][0] as i64 - nums[left][0] as i64);
                ans = max(ans, tamp * nums[right][2] as i64 + presum[right] - presum[left]);
                left += 1;
            }
            if left > nums.len() {
                break;
            }
            ans = max(ans, presum[right + 1] - presum[left]);
            right += 1;
        }

        left = nums.len() - 1;
        right = nums.len() - 1;

        while right >= 0 && left >= 0 {
            while right >= 0 && (nums[right][1] as i64 - nums[left][0] as i64 + 1) > k {
                let tamp: i64 = k - (nums[right][1] as i64 - nums[left][1] as i64);
                ans = max(ans, tamp * nums[left][2] as i64 + presum[right + 1] - presum[left + 1]);
                if right == 0 { break; }
                right -= 1;
            }
            if right < 0 {
                break;
            }
            ans = max(ans, presum[right + 1] - presum[left]);
            if left == 0 { break; }
            left -= 1;
        }

        ans
    }
}