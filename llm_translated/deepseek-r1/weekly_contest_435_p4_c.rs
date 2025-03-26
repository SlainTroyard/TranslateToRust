use std::cmp::{max, min};
use std::io;

fn max_difference(s: &str, k: i32) -> i32 {
    let digits: Vec<usize> = s
        .chars()
        .map(|c| (c as u8 - b'0') as usize)
        .collect();
    let len = digits.len();
    let mut ans = i32::MIN / 2;

    // Iterate through all possible pairs of digits (x,y) where x != y
    for x in 0..5 {
        for y in 0..5 {
            if x == y {
                continue;
            }

            // Initialize counters and tracking variables for current window
            let mut cur_s = [0i32; 5];  // Counts of each digit in current window
            let mut pre_s = [0i32; 5];  // Counts of digits left of the window
            let mut min_s = [[i32::MAX / 2; 2]; 2];  // Minimum differences by parity
            let mut left = 0;  // Left boundary of the window

            // Process each character in the string
            for i in 0..len {
                let d = digits[i];
                cur_s[d] += 1;  // Expand window to include current character
                let r = i + 1;

                // Shrink window from left while conditions are met
                while (r - left) >= k as usize 
                    && cur_s[x] > pre_s[x] 
                    && cur_s[y] > pre_s[y] 
                {
                    // Update minimum difference based on parity of pre_s counts
                    let px = pre_s[x];
                    let py = pre_s[y];
                    let px_parity = (px & 1) as usize;
                    let py_parity = (py & 1) as usize;
                    min_s[px_parity][py_parity] = min(min_s[px_parity][py_parity], px - py);

                    // Move left boundary and update pre_s counts
                    let left_d = digits[left];
                    pre_s[left_d] += 1;
                    left += 1;
                }

                // Calculate current potential maximum difference
                let current_val = cur_s[x] - cur_s[y];
                let inv_x_parity = ((cur_s[x] & 1) ^ 1) as usize;  // Inverted parity for x
                let y_parity = (cur_s[y] & 1) as usize;
                let min_val = min_s[inv_x_parity][y_parity];
                ans = max(ans, current_val - min_val);
            }
        }
    }

    ans
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    if parts.len() != 2 {
        eprintln!("Error reading input");
        return;
    }

    let s = parts[0];
    let k: i32 = parts[1].parse().expect("Invalid integer for k");

    let result = max_difference(s, k);
    println!("{}", result);
}