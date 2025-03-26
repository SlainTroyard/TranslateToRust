fn min_difference(nums: &[i32]) -> i32 {
    let n = nums.len();
    let mut min_l = i32::MAX;
    let mut max_r = 0;

    // Find min_l and max_r by checking for the adjacent '-1' elements
    for i in 0..n {
        if nums[i] != -1 {
            if (i > 0 && nums[i - 1] == -1) || (i < n - 1 && nums[i + 1] == -1) {
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

    // Helper function to update the result (same logic as the C++ lambda function)
    fn update_ans(l: i32, r: i32, big: bool, min_l: i32, max_r: i32, ans: &mut i32) {
        let mut d = if (r - min_l) < (max_r - l) {
            r - min_l
        } else {
            max_r - l
        } + 1;
        d /= 2;
        if big {
            d = if d < (max_r - min_l + 2) / 3 {
                d
            } else {
                (max_r - min_l + 2) / 3
            };
        }
        if d > *ans {
            *ans = d;
        }
    }

    // Calculate the answer by iterating through the elements
    let mut pre_i: Option<usize> = None;
    for i in 0..n {
        if nums[i] == -1 {
            continue;
        }
        if let Some(pre_index) = pre_i {
            if i - pre_index == 1 {
                let diff = (nums[i] - nums[pre_index]).abs();
                if diff > ans {
                    ans = diff;
                }
            } else {
                let l = if nums[pre_index] < nums[i] { nums[pre_index] } else { nums[i] };
                let r = if nums[pre_index] > nums[i] { nums[pre_index] } else { nums[i] };
                update_ans(l, r, (i - pre_index) > 2, min_l, max_r, &mut ans);
            }
        } else if i > 0 {
            update_ans(nums[i], nums[i], false, min_l, max_r, &mut ans);
        }
        pre_i = Some(i);
    }

    if let Some(pre_index) = pre_i {
        if pre_index < n - 1 {
            update_ans(nums[pre_index], nums[pre_index], false, min_l, max_r, &mut ans);
        }
    }

    ans
}

fn main() {
    let mut n_str = String::new();
    std::io::stdin().read_line(&mut n_str).expect("Failed to read line");
    let n: usize = n_str.trim().parse().expect("Please type a number!");

    let mut nums_str = String::new();
    std::io::stdin().read_line(&mut nums_str).expect("Failed to read line");
    let nums: Vec<i32> = nums_str
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Please type a number!"))
        .collect();

    let result = min_difference(&nums);
    println!("{}", result);
}