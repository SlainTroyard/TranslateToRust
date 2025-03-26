use std::io;

struct Solution;

impl Solution {
    fn maximum_coins(&self, mut nums: Vec<Vec<i32>>, k: i64) -> i64 {
        // Sort by the first element
        nums.sort_by_key(|v| v[0]);
        let n = nums.len();
        let mut presum = vec![0i64; n + 1];
        for i in 1..=n {
            let prev = presum[i - 1];
            let current = (nums[i - 1][1] - nums[i - 1][0] + 1) as i64 * nums[i - 1][2] as i64;
            presum[i] = prev + current;
        }
        let mut ans: i64 = 0;
        let mut left: usize = 0;
        let mut right: usize = 0;
        while right < n && left < n {
            while left < n && (nums[right][1] as i64 - nums[left][0] as i64 + 1) > k {
                let tamp = k - (nums[right][0] as i64 - nums[left][0] as i64);
                ans = ans.max(tamp * nums[right][2] as i64 + (presum[right] - presum[left]));
                left += 1;
            }
            if left > n {
                break;
            }
            ans = ans.max(presum[right + 1] - presum[left]);
            right += 1;
        }
        // Second loop
        let mut left: isize = (n as isize) - 1;
        let mut right: isize = (n as isize) - 1;
        while right >= 0 && left >= 0 {
            while right >= 0 && (nums[right as usize][1] as i64 - nums[left as usize][0] as i64 + 1) > k {
                let tamp = k - (nums[right as usize][1] as i64 - nums[left as usize][1] as i64);
                ans = ans.max(tamp * nums[left as usize][2] as i64 + (presum[right as usize + 1] - presum[left as usize + 1]));
                right -= 1;
            }
            if right < 0 {
                break;
            }
            ans = ans.max(presum[right as usize + 1] - presum[left as usize]);
            left -= 1;
        }
        ans
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let n = tokens.next().unwrap();
    let k = tokens.next().unwrap() as i64;
    let mut nums = Vec::new();
    for _ in 0..n {
        let a = tokens.next().unwrap();
        let b = tokens.next().unwrap();
        let c = tokens.next().unwrap();
        nums.push(vec![a, b, c]);
    }
    let solution = Solution {};
    println!("{}", solution.maximum_coins(nums, k));
}