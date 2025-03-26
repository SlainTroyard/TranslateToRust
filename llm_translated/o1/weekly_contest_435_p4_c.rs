use std::cmp::{max, min};
use std::io::{self, Read};
use std::process;

/// Computes the maximum difference based on the given problem logic.
fn max_difference(s: &str, k: usize) -> i32 {
    let inf = i32::MAX / 2;
    let mut ans = -inf;
    let len = s.len();

    // Iterate over all possible pairs (x, y) where x != y, and x, y in [0..4]
    for x in 0..5 {
        for y in 0..5 {
            if x == y {
                continue;
            }

            // Track the count of digits in the current window and the "left portion" window
            let mut cur_s = [0i32; 5];
            let mut pre_s = [0i32; 5];

            // min_s stores the minimum values of (pre_s[x] - pre_s[y]) for parity combinations
            let mut min_s = [[inf, inf], [inf, inf]];

            // Left index of the window
            let mut left = 0usize;

            // Expand the window by moving i
            for i in 0..len {
                // Convert the i-th character to a digit
                let digit = s.chars().nth(i).unwrap().to_digit(10).unwrap() as usize;
                cur_s[digit] += 1;
                let r = i + 1;

                // Try to shrink the window from the left while conditions are satisfied
                while r - left >= k
                    && cur_s[x] > pre_s[x]
                    && cur_s[y] > pre_s[y]
                {
                    // Update the minimum difference for the current parity of pre_s[x], pre_s[y]
                    let px = pre_s[x] & 1;
                    let py = pre_s[y] & 1;
                    min_s[px as usize][py as usize] = min(
                        min_s[px as usize][py as usize],
                        pre_s[x] - pre_s[y],
                    );

                    // Move the left pointer, updating the pre_s counts
                    let left_digit = s.chars().nth(left).unwrap().to_digit(10).unwrap() as usize;
                    pre_s[left_digit] += 1;
                    left += 1;
                }

                // Update the final answer using the current parity of cur_s[x], cur_s[y]
                let cx = cur_s[x];
                let cy = cur_s[y];
                let idx_x = ((cx & 1) ^ 1) as usize; // toggled parity
                let idx_y = (cy & 1) as usize;       // current parity
                ans = max(ans, cx - cy - min_s[idx_x][idx_y]);
            }
        }
    }

    ans
}

fn main() {
    // Read all input from stdin
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        eprintln!("Error reading input");
        process::exit(1);
    }

    // Split by whitespace to simulate scanf("%s %d", ...)
    let tokens: Vec<&str> = input.split_whitespace().collect();
    if tokens.len() < 2 {
        eprintln!("Error reading input");
        process::exit(1);
    }

    // Parse the string s and integer k
    let s = tokens[0];
    let k: i32 = match tokens[1].parse() {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Error reading input");
            process::exit(1);
        }
    };

    // Compute the result and print to stdout
    let result = max_difference(s, k as usize);
    println!("{}", result);
}