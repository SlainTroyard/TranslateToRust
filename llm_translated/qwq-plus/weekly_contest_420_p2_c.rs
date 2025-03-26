use std::io;

/// Translated C code from LeetCode Weekly Contest 420 Problem 2 to idiomatic Rust.
///
/// The function counts the number of substrings in a string `s` that contain exactly `k` distinct characters.
/// The algorithm uses a sliding window approach with a character frequency array to track valid substrings.
///
/// # Arguments
///
/// * `s` - A string slice representing the input string.
/// * `k` - An integer representing the required number of distinct characters.
///
/// # Returns
///
/// * The count of valid substrings as an integer.
fn number_of_substrings(s: &str, k: i32) -> i32 {
    let s_l = s.len();
    let mut hash_arr: [usize; 26] = [0; 26];
    let mut left = 0;
    let mut right = 0;
    let mut res: i32 = 0;

    while left < s_l && right < s_l {
        let current_char = (s.as_bytes()[right] - b'a') as usize;
        hash_arr[current_char] += 1;

        // Check if the current character's count reaches exactly `k`
        if hash_arr[current_char] == k as usize {
            res += (s_l - right) as i32;

            // Adjust the left pointer to maintain the window constraint
            while left <= right {
                let c = (s.as_bytes()[left] - b'a') as usize;
                hash_arr[c] -= 1;
                left += 1;
                if hash_arr[c] != (k as usize - 1) {
                    res += (s_l - right) as i32;
                } else {
                    break;
                }
            }
            right += 1;
        } else {
            right += 1;
        }
    }

    res
}

/// Main function to handle input and output.
///
/// Reads input from stdin in the format "string integer", processes it using `number_of_substrings`,
/// and prints the result to stdout.
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let parts: Vec<&str> = input.trim().splitn(2, ' ').collect();
    if parts.len() != 2 {
        panic!("Input must be a string followed by an integer");
    }
    let s = parts[0];
    let k = parts[1].parse().expect("Invalid integer");
    println!("{}", number_of_substrings(s, k));
}