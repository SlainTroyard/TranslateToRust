use std::io::{self, Read};

struct Solution;

impl Solution {
    pub fn min_diff(&self, nums: &[i32]) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }

        let mut min_l = i32::MAX;
        let mut max_r = 0;

        // Find min and max of elements adjacent to -1
        for i in 0..n {
            if nums[i] == -1 {
                continue;
            }
            let adjacent = (i > 0 && nums[i - 1] == -1) || (i < n - 1 && nums[i + 1] == -1);
            if adjacent {
                min_l = min_l.min(nums[i]);
                max_r = max_r.max(nums[i]);
            }
        }

        let mut ans = 0;
        let min_l = min_l;
        let max_r = max_r;

        // Closure to update answer based on current boundaries
        let mut update_ans = |l: i32, r: i32, big: bool| {
            let d = ((r - min_l).min(max_r - l) + 1) / 2;
            let d = if big {
                d.min((max_r - min_l + 2) / 3)
            } else {
                d
            };
            ans = ans.max(d);
        };

        let mut pre_i = None; // Track previous non--1 index

        for i in 0..n {
            if nums[i] == -1 {
                continue;
            }

            match pre_i {
                Some(prev) => {
                    if i - prev == 1 {
                        ans = ans.max((nums[i] - nums[prev]).abs());
                    } else {
                        let (l, r) = (nums[prev].min(nums[i]), nums[prev].max(nums[i]));
                        let big = i - prev > 2;
                        update_ans(l, r, big);
                    }
                }
                None => {
                    if i > 0 {
                        update_ans(nums[i], nums[i], false);
                    }
                }
            }

            pre_i = Some(i);
        }

        // Handle trailing -1 elements after last non--1
        if let Some(p) = pre_i {
            if p < n - 1 {
                update_ans(nums[p], nums[p], false);
            }
        }

        ans
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut tokens = input.split_whitespace();

    let n: usize = tokens.next().expect("No input")
        .parse().expect("Invalid n");
    let nums: Vec<i32> = tokens.take(n)
        .map(|s| s.parse().expect("Invalid number"))
        .collect();

    assert_eq!(nums.len(), n, "Insufficient numbers provided");
    
    let sol = Solution;
    println!("{}", sol.min_diff(&nums));
}