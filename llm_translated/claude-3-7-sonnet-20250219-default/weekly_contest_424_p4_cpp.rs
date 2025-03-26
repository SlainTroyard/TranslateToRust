use std::io::{self, BufRead};
use std::cmp::{min, max};

struct Solution;

impl Solution {
    pub fn min_difference(nums: &Vec<i32>) -> i32 {
        let n = nums.len();
        let mut min_l = i32::MAX;
        let mut max_r = 0;
        
        // Find min and max values adjacent to -1s
        for i in 0..n {
            if nums[i] != -1 && ((i > 0 && nums[i - 1] == -1) || (i < n - 1 && nums[i + 1] == -1)) {
                min_l = min(min_l, nums[i]);
                max_r = max(max_r, nums[i]);
            }
        }

        let mut ans = 0;
        let mut update_ans = |l: i32, r: i32, big: bool| {
            let mut d = (min(r - min_l, max_r - l) + 1) / 2;
            if big {
                d = min(d, (max_r - min_l + 2) / 3);
            }
            ans = max(ans, d);
        };

        let mut pre_i: i32 = -1;
        for i in 0..n {
            if nums[i] == -1 {
                continue;
            }
            if pre_i >= 0 {
                if i as i32 - pre_i == 1 {
                    ans = max(ans, (nums[i] - nums[pre_i as usize]).abs());
                } else {
                    update_ans(
                        min(nums[pre_i as usize], nums[i]),
                        max(nums[pre_i as usize], nums[i]),
                        i as i32 - pre_i > 2,
                    );
                }
            } else if i > 0 {
                update_ans(nums[i], nums[i], false);
            }
            pre_i = i as i32;
        }
        
        if 0 <= pre_i && pre_i < n as i32 - 1 {
            update_ans(nums[pre_i as usize], nums[pre_i as usize], false);
        }
        
        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the number of elements
    let n: usize = lines.next().unwrap().unwrap().trim().parse().expect("Invalid input for n");
    
    // Read the nums array
    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input for nums"))
        .collect();
    
    // Check if input size matches the expected size
    assert_eq!(nums.len(), n, "Input size mismatch");
    
    let sol = Solution;
    let result = Solution::min_difference(&nums);
    
    println!("{}", result);
}