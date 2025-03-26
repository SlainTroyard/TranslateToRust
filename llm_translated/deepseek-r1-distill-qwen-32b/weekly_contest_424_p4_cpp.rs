struct Solution {}

impl Solution {
    fn min_difference(&self, nums: &mut Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }

        let mut min_l = i32::MAX;
        let mut max_r = 0;

        for i in 0..n {
            if nums[i] == -1 {
                continue;
            }

            let left = if i > 0 { nums[i - 1] == -1 } else { false };
            let right = if i < n - 1 { nums[i + 1] == -1 } else { false };

            if left || right {
                if nums[i] < min_l {
                    min_l = nums[i];
                }
                if nums[i] > max_r {
                    max_r = nums[i];
                }
            }
        }

        let mut ans = 0;

        let min_l = min_l;
        let max_r = max_r;

        let mut update_ans = move |l: i32, r: i32, big: bool| {
            let d = (r - min_l).min(max_r - l) + 1;
            let mut d = d / 2;
            if big {
                let option = (max_r - min_l + 2) / 3;
                d = d.min(option);
            }
            ans = ans.max(d);
        };

        let mut pre_i: isize = -1;

        for i in 0..n {
            if nums[i] == -1 {
                continue;
            }

            if pre_i >= 0 {
                let gap = i - pre_i as usize;
                if gap == 1 {
                    let diff = (nums[i] - nums[pre_i as usize]).abs();
                    ans = ans.max(diff);
                } else {
                    let l = nums[pre_i as usize].min(nums[i]);
                    let r = nums[pre_i as usize].max(nums[i]);
                    let big = gap > 2;
                    update_ans(l, r, big);
                }
            } else {
                if i > 0 {
                    update_ans(nums[i], nums[i], false);
                }
            }

            pre_i = i as isize;
        }

        if pre_i >= 0 && pre_i < (n as isize - 1) {
            update_ans(nums[pre_i as usize], nums[pre_i as usize], false);
        }

        ans
    }
}

fn main() {
    use std::io;

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut tokens = input.split_whitespace();
    let n: usize = tokens.next().unwrap().parse().unwrap();
    let nums: Vec<i32> = tokens.take(n).map(|x| x.parse().unwrap()).collect();

    let sol = Solution {};
    let result = sol.min_difference(&mut nums);

    println!("{}", result);
}