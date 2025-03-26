use std::cmp;
use std::io::{self, BufRead};

/// This function implements the algorithm from CPP.
/// It returns the maximum difference computed based on the input parameters.
fn max_difference(s: &str, k: i32) -> i32 {
    // Define infinity as INT_MAX/2 like in C++
    let inf: i32 = std::i32::MAX / 2;
    let mut ans = -inf;
    let s_bytes = s.as_bytes();
    let n = s_bytes.len();

    // Try all pairs of digits (x, y) with 0 <= x, y < 5 and x != y.
    for x in 0..5 {
        for y in 0..5 {
            if x == y {
                continue;
            }
            // current frequency counter for each digit (0..4)
            let mut cur_s = [0i32; 5];
            // prefix frequency counter for each digit (0..4)
            let mut pre_s = [0i32; 5];

            // min_s is a 2x2 array. We use parity (even/odd) of counts.
            let mut min_s = [[inf; 2]; 2];
            let mut left = 0;
            // Loop over each character in the string. In C++ i is index.
            for i in 0..n {
                // Convert the current byte to digit.
                // Since we know the characters are digits, subtract b'0'
                let d = (s_bytes[i] - b'0') as usize;
                cur_s[d] += 1;
                let r = (i + 1) as i32;

                // Check the while condition:
                // (r - left >= k) and (cur_s[x] > pre_s[x]) and (cur_s[y] > pre_s[y])
                while r - (left as i32) >= k && cur_s[x] > pre_s[x] && cur_s[y] > pre_s[y] {
                    // Use parity of pre_s[x] and pre_s[y] to update the corresponding entry in min_s.
                    let px_parity = (pre_s[x] & 1) as usize;
                    let py_parity = (pre_s[y] & 1) as usize;
                    // Update the min value with the difference pre_s[x] - pre_s[y]
                    min_s[px_parity][py_parity] = cmp::min(min_s[px_parity][py_parity], pre_s[x] - pre_s[y]);

                    // Update pre_s with the character at the left pointer.
                    let left_digit = (s_bytes[left] - b'0') as usize;
                    pre_s[left_digit] += 1;
                    left += 1;
                }
                // Compute the parity indices for current frequency.
                // (cur_s[x] & 1) ^ 1 flips the parity bit.
                let cur_px_parity = ((cur_s[x] & 1) ^ 1) as usize;
                let cur_py_parity = (cur_s[y] & 1) as usize;

                // Update ans with the computed difference.
                ans = cmp::max(ans, cur_s[x] - cur_s[y] - min_s[cur_px_parity][cur_py_parity]);
            }
        }
    }
    ans
}

fn main() -> io::Result<()> {
    // Prepare to read from stdin.
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut input = String::new();

    // Read the entire input.
    reader.read_to_string(&mut input)?;

    // Split input by whitespace.
    let mut tokens = input.split_whitespace();

    // Extract the string s.
    let s = match tokens.next() {
        Some(token) => token,
        None => {
            eprintln!("Expected a string input");
            return Ok(());
        }
    };

    // Extract integer k.
    let k: i32 = match tokens.next() {
        Some(token) => token.parse().unwrap_or_else(|_| {
            eprintln!("Failed to parse integer k");
            0
        }),
        None => {
            eprintln!("Expected an integer input for k");
            0
        }
    };

    // Compute and print the answer.
    let result = max_difference(s, k);
    println!("{}", result);

    Ok(())
}