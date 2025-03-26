use std::io::{self, BufRead, Write};

fn min_difference(nums: &mut Vec<i32>) -> i32 {
    let n = nums.len();
    let mut min_l = i32::MAX;
    let mut max_r = 0;

    for i in 0..n {
        if nums[i] != -1 && (i > 0 && nums[i - 1] == -1 || i < n - 1 && nums[i + 1] == -1) {
            min_l = min_l.min(nums[i]);
            max_r = max_r.max(nums[i]);
        }
    }

    let mut ans = 0;
    let update_ans = |l: i32, r: i32, big: bool| {
        let d = ((r - min_l).min(max_r - l) + 1) / 2;
        let d = if big { d.min((max_r - min_l + 2) / 3) } else { d };
        ans = ans.max(d);
    };

    let mut pre_i = -1;
    for i in 0..n {
        if nums[i] == -1 {
            continue;
        }
        if pre_i >= 0 {
            if i - pre_i == 1 {
                ans = ans.max((nums[i] - nums[pre_i]).abs());
            } else {
                update_ans(nums[pre_i].min(nums[i]), nums[pre_i].max(nums[i]), i - pre_i > 2);
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

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer = String::new();

    // Read the number of elements
    stdin.lock().read_line(&mut buffer).expect("Failed to read line");
    let n: usize = buffer.trim().parse().expect("Please enter a valid number");
    buffer.clear();

    // Read the elements into a vector
    let mut nums = vec![0; n];
    for num in nums.iter_mut() {
        stdin.lock().read_line(&mut buffer).expect("Failed to read line");
        *num = buffer.trim().parse().expect("Please enter a valid number");
        buffer.clear();
    }

    // Calculate the result
    let result = min_difference(&mut nums);

    // Print the result
    writeln!(stdout, "{}", result).expect("Failed to write to stdout");
}