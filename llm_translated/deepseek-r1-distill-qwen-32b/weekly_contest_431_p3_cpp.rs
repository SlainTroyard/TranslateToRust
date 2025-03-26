use std::cmp::max;
use std::io;

struct Solution {}

impl Solution {
    pub fn maximum_coins(nums: Vec<Vec<i32>>, k: i32) -> i64 {
        let mut nums = nums.clone();
        nums.sort_by(|a, b| a[0].cmp(&b[0]));
        
        let n = nums.len();
        let mut presum = vec![0; n + 1];
        for i in 1..=n {
            let val = (nums[i-1][1] - nums[i-1][0] + 1) as i64 * nums[i-1][2] as i64;
            presum[i] = presum[i-1] + val;
        }
        
        let mut ans = 0;
        let mut left = 0;
        let mut right = 0;
        while right < n && left < n {
            while left < n && (nums[right][1] - nums[left][0] + 1) > k {
                let tamp = (k - (nums[right][0] - nums[left][0])) as i64;
                let current = tamp * nums[right][2] as i64 + (presum[right] - presum[left]);
                ans = max(ans, current);
                left += 1;
            }
            if left > n {
                break;
            }
            let current = presum[right + 1] - presum[left];
            ans = max(ans, current);
            right += 1;
        }
        
        left = n - 1;
        right = n - 1;
        while right >= 0 && left >= 0 {
            while right >= 0 && (nums[right][1] - nums[left][0] + 1) > k {
                let tamp = (k - (nums[right][1] - nums[left][1])) as i64;
                let current = tamp * nums[left][2] as i64 + (presum[right + 1] - presum[left + 1]);
                ans = max(ans, current);
                right -= 1;
            }
            if right < 0 {
                break;
            }
            let current = presum[right + 1] - presum[left];
            ans = max(ans, current);
            left -= 1;
        }
        
        ans
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut lines = input.lines();
    
    let first_line = lines.next().expect("No input");
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().expect("Invalid n");
    let k: i32 = parts.next().unwrap().parse().expect("Invalid k");
    
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().expect("Not enough lines");
        let mut parts = line.split_whitespace();
        let a: i32 = parts.next().unwrap().parse().expect("Invalid a");
        let b: i32 = parts.next().unwrap().parse().expect("Invalid b");
        let c: i32 = parts.next().unwrap().parse().expect("Invalid c");
        nums.push(vec![a, b, c]);
    }
    
    let sol = Solution {};
    println!("{}", sol.maximum_coins(nums, k));
}