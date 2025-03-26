use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    // Set up a buffered reader for stdin.
    let stdin = io::stdin();
    let mut reader = stdin.lock().lines();

    // Read the first line which contains the integer n.
    // It might be the entire line or there may be extra tokens, so we use split_whitespace.
    let n = loop {
        if let Some(Ok(line)) = reader.next() {
            // Try to parse the first token as n; skip empty lines.
            if !line.trim().is_empty() {
                if let Some(token) = line.split_whitespace().next() {
                    match token.parse::<usize>() {
                        Ok(val) => break val,
                        Err(_) => {
                            eprintln!("Error parsing input for n.");
                            return Ok(());
                        }
                    }
                }
            }
        } else {
            eprintln!("Unexpected end of input while reading n.");
            return Ok(());
        }
    };
    
    // Now, read n numbers. They can be on multiple lines or on a single line.
    let mut nums = Vec::with_capacity(n);
    while nums.len() < n {
        if let Some(Ok(line)) = reader.next() {
            // Split the line into tokens
            for token in line.split_whitespace() {
                if nums.len() >= n { break; }
                match token.parse::<i32>() {
                    Ok(val) => nums.push(val),
                    Err(_) => {
                        eprintln!("Error parsing an integer from input.");
                        return Ok(());
                    }
                }
            }
        } else {
            break;
        }
    }
    if nums.len() < n {
        eprintln!("Not enough numbers in input.");
        return Ok(());
    }
    
    let result = min_difference(&nums);
    println!("{}", result);
    Ok(())
}

// Function that calculates the minimum difference according to the problem logic.
fn min_difference(nums: &[i32]) -> i32 {
    let n = nums.len();
    // Use i32::MAX for INT_MAX and 0 as the starting value for max_r.
    let mut min_l = i32::MAX;
    let mut max_r = 0;
    
    // First pass: find min_l and max_r by checking adjacent -1's.
    for i in 0..n {
        if nums[i] != -1
            && ((i > 0 && nums[i - 1] == -1) || (i < n - 1 && nums[i + 1] == -1))
        {
            if nums[i] < min_l {
                min_l = nums[i];
            }
            if nums[i] > max_r {
                max_r = nums[i];
            }
        }
    }
    
    let mut ans = 0;
    
    // Define a helper closure that updates ans with the calculated difference.
    let mut update_ans = |l: i32, r: i32, big: bool| {
        // Calculate d as in the C code:
        // d = (min(r - min_l, max_r - l) + 1) / 2
        let base = (r - min_l).min(max_r - l) + 1;
        let mut d = base / 2;
        if big {
            // If big flag is set, update d to be the minimum of d and (max_r - min_l + 2) / 3.
            d = d.min((max_r - min_l + 2) / 3);
        }
        if d > ans {
            ans = d;
        }
    };
    
    let mut pre_i: i32 = -1; // previous index that is not -1
    
    for i in 0..n as i32 {
        let i_usize = i as usize;
        if nums[i_usize] == -1 {
            continue;
        }
        if pre_i >= 0 {
            let pre_idx = pre_i as usize;
            if (i - pre_i) == 1 {
                // Consecutive elements: update ans with the absolute difference.
                let diff = (nums[i_usize] - nums[pre_idx]).abs();
                if diff > ans {
                    ans = diff;
                }
            } else {
                // Calculate update_ans with the min and max between nums[pre_i] and nums[i]
                let (l, r) = if nums[pre_idx] < nums[i_usize] {
                    (nums[pre_idx], nums[i_usize])
                } else {
                    (nums[i_usize], nums[pre_idx])
                };
                update_ans(l, r, (i - pre_i) > 2);
            }
        } else if i > 0 {
            // This case handles the beginning part.
            update_ans(nums[i_usize], nums[i_usize], false);
        }
        pre_i = i;
    }
    
    // After iterating, if the last valid index is before the end, update_ans one final time.
    if pre_i >= 0 && (pre_i as usize) < n - 1 {
        update_ans(nums[pre_i as usize], nums[pre_i as usize], false);
    }
    
    ans
}