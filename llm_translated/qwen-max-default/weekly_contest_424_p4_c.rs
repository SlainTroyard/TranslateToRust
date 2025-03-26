use std::io::{self, BufRead, Write};

fn min_difference(nums: &mut [i32]) -> i32 {
    let n = nums.len();
    let mut min_l = i32::MAX;
    let mut max_r = 0;

    // Find min_l and max_r by checking for the adjacent '-1' elements
    for i in 0..n {
        if nums[i] != -1 && 
           (i > 0 && nums[i - 1] == -1 || i < n - 1 && nums[i + 1] == -1) {
            if nums[i] < min_l { min_l = nums[i]; }
            if nums[i] > max_r { max_r = nums[i]; }
        }
    }

    let mut ans = 0;

    // Helper function to update the result
    fn update_ans(l: i32, r: i32, big: bool, ans: &mut i32) {
        let d = if r - min_l < max_r - l { r - min_l } else { max_r - l } + 1;
        let d = d / 2;
        let d = if big { d.min((max_r - min_l + 2) / 3) } else { d };
        if d > *ans { *ans = d; }
    }

    // Calculate the answer by iterating through the elements
    let mut pre_i = -1;
    for i in 0..n {
        if nums[i] == -1 {
            continue;
        }
        if pre_i >= 0 {
            if i - pre_i == 1 {
                let diff = (nums[i] - nums[pre_i]).abs();
                if diff > ans { ans = diff; }
            } else {
                update_ans(
                    nums[pre_i].min(nums[i]),
                    nums[pre_i].max(nums[i]),
                    i - pre_i > 2,
                    &mut ans,
                );
            }
        } else if i > 0 {
            update_ans(nums[i], nums[i], false, &mut ans);
        }
        pre_i = i as i32;
    }

    if 0 <= pre_i && pre_i < n as i32 - 1 {
        update_ans(nums[pre_i as usize], nums[pre_i as usize], false, &mut ans);
    }

    ans
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let stdout = io::stdout();
    let mut stdout_lock = stdout.lock();

    let mut input = String::new();
    stdin_lock.read_line(&mut input)?;
    let n: usize = input.trim().parse().expect("Invalid input");

    let mut nums = vec![0; n];
    for i in 0..n {
        input.clear();
        stdin_lock.read_line(&mut input)?;
        nums[i] = input.trim().parse().expect("Invalid input");
    }

    let result = min_difference(&mut nums);
    writeln!(stdout_lock, "{}", result)?;

    Ok(())
}