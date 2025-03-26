use std::io::{self, Read};

fn min_difference(nums: &[i32]) -> i32 {
    let n = nums.len();
    let mut min_l = i32::MAX;
    let mut max_r = 0;

    // Find min_l and max_r for elements adjacent to -1
    for i in 0..n {
        if nums[i] != -1 {
            let adjacent = (i > 0 && nums[i-1] == -1) || (i < n-1 && nums[i+1] == -1);
            if adjacent {
                if nums[i] < min_l {
                    min_l = nums[i];
                }
                if nums[i] > max_r {
                    max_r = nums[i];
                }
            }
        }
    }

    let mut ans = 0;

    // Closure to compute the possible d value
    let compute_d = |l: i32, r: i32, big: bool| -> i32 {
        let mut d = (if (r - min_l) < (max_r - l) {
            r - min_l
        } else {
            max_r - l
        } + 1) / 2;
        if big {
            let temp = (max_r - min_l + 2) / 3;
            d = d.min(temp);
        }
        d
    };

    let mut pre_i: Option<usize> = None;

    // Iterate through the array to compute the answer
    for i in 0..n {
        if nums[i] == -1 {
            continue;
        }
        if let Some(pre) = pre_i {
            if i - pre == 1 {
                // Consecutive elements, check absolute difference
                let diff = (nums[i] - nums[pre]).abs();
                ans = ans.max(diff);
            } else {
                // Non-consecutive elements, use helper logic
                let l = nums[pre].min(nums[i]);
                let r = nums[pre].max(nums[i]);
                let big = i - pre > 2;
                let d = compute_d(l, r, big);
                ans = ans.max(d);
            }
        } else if i > 0 {
            // First valid element after some -1's
            let l = nums[i];
            let r = nums[i];
            let d = compute_d(l, r, false);
            ans = ans.max(d);
        }
        pre_i = Some(i);
    }

    // Handle trailing -1 elements
    if let Some(pre) = pre_i {
        if pre < n - 1 {
            let l = nums[pre];
            let r = nums[pre];
            let d = compute_d(l, r, false);
            ans = ans.max(d);
        }
    }

    ans
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut tokens = input.split_whitespace();
    let n: usize = tokens.next().unwrap().parse().unwrap();
    let nums: Vec<i32> = tokens.take(n).map(|s| s.parse().unwrap()).collect();
    println!("{}", min_difference(&nums));
}