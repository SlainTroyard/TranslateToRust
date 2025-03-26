struct Solution;

impl Solution {
    fn min_difference(&self, nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut min_l = i32::MAX;
        let mut max_r = 0;
        for (i, &num) in nums.iter().enumerate() {
            if num == -1 {
                continue;
            }
            let has_left = if i > 0 { nums[i - 1] == -1 } else { false };
            let has_right = if i < n - 1 { nums[i + 1] == -1 } else { false };
            if has_left || has_right {
                if num < min_l {
                    min_l = num;
                }
                if num > max_r {
                    max_r = num;
                }
            }
        }

        let mut ans = 0;

        // Define the helper function as a closure
        let update_ans = |l: i32, r: i32, big: bool, current_ans: i32| -> i32 {
            let d = (std::cmp::min(r - min_l, max_r - l) + 1) / 2;
            let mut d_val = d;
            if big {
                d_val = std::cmp::min(d_val, (max_r - min_l + 2) / 3);
            }
            current_ans.max(d_val)
        };

        let mut pre_i: Option<usize> = None;
        for (i, &num) in nums.iter().enumerate() {
            if num == -1 {
                continue;
            }
            if let Some(prev) = pre_i {
                let distance = i - prev;
                if distance == 1 {
                    ans = ans.max((num - nums[prev]).abs());
                } else {
                    let l = nums[prev].min(num);
                    let r = nums[prev].max(num);
                    let big = distance > 2;
                    ans = update_ans(l, r, big, ans);
                }
            } else if i > 0 {
                ans = update_ans(num, num, false, ans);
            }
            pre_i = Some(i);
        }

        if let Some(last) = pre_i {
            if last < n - 1 {
                ans = update_ans(nums[last], nums[last], false, ans);
            }
        }

        ans
    }
}

fn main() {
    use std::io;

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut parts = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());
    let n = parts.next().unwrap() as usize;
    let nums: Vec<i32> = parts.take(n).collect();
    let sol = Solution;
    let result = sol.min_difference(nums);
    println!("{}", result);
}