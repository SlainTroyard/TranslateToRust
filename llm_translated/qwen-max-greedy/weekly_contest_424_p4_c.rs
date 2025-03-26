use std::io::{self, BufRead, Write};

fn min_difference(nums: &Vec<i32>) -> i32 {
    let mut min_l = i32::MAX;
    let mut max_r = 0;

    // Find min_l and max_r by checking for the adjacent '-1' elements
    for (i, &num) in nums.iter().enumerate() {
        if num != -1 && 
           ((i > 0 && nums[i - 1] == -1) || (i < nums.len() - 1 && nums[i + 1] == -1)) {
            if num < min_l { min_l = num; }
            if num > max_r { max_r = num; }
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
    for (i, &num) in nums.iter().enumerate() {
        if num == -1 {
            continue;
        }
        if pre_i >= 0 {
            if i - pre_i == 1 {
                let diff = (nums[pre_i as usize] - num).abs();
                if diff > ans { ans = diff; }
            } else {
                let (l, r) = if nums[pre_i as usize] < num {
                    (nums[pre_i as usize], num)
                } else {
                    (num, nums[pre_i as usize])
                };
                update_ans(l, r, i - pre_i > 2, &mut ans);
            }
        } else if i > 0 {
            update_ans(num, num, false, &mut ans);
        }
        pre_i = i as i32;
    }

    if 0 <= pre_i && pre_i < nums.len() as i32 - 1 {
        update_ans(nums[pre_i as usize], nums[pre_i as usize], false, &mut ans);
    }

    ans
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer = String::new();

    // Read the number of elements
    stdin.lock().read_line(&mut buffer)?;
    let n: usize = buffer.trim().parse().expect("Failed to parse n");
    buffer.clear();

    // Read the elements
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        buffer.clear();
        stdin.lock().read_line(&mut buffer)?;
        nums.push(buffer.trim().parse().expect("Failed to parse element"));
    }

    // Calculate the result
    let result = min_difference(&nums);

    // Print the result
    writeln!(stdout, "{}", result)?;

    Ok(())
}