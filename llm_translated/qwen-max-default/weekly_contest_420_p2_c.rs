use std::io::{self, BufRead};

/// Counts the number of substrings with exactly `k` occurrences of a character.
fn number_of_substrings(s: &str, k: usize) -> usize {
    let mut hash_arr = [0; 26];
    let s_bytes = s.as_bytes();
    let s_l = s.len();
    let mut left = 0;
    let mut right = 0;
    let mut res = 0;

    while left < s_l && right < s_l {
        if hash_arr[(s_bytes[right] - b'a') as usize] + 1 == k {
            res += s_l - right;
            while left <= right {
                if hash_arr[(s_bytes[left] - b'a') as usize] - 1 != k - 1 {
                    res += s_l - right;
                } else {
                    break;
                }
                hash_arr[(s_bytes[left] - b'a') as usize] -= 1;
                left += 1;
            }
            right += 1;
        } else {
            hash_arr[(s_bytes[right] - b'a') as usize] += 1;
            right += 1;
        }
    }

    res
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Parse the input string and integer
    let line = lines.next().unwrap().expect("Failed to read input");
    let mut iter = line.split_whitespace();
    let s = iter.next().unwrap();
    let k: usize = iter.next().unwrap().parse().expect("Failed to parse integer");

    // Calculate the result
    let result = number_of_substrings(s, k);

    // Print the result to stdout
    println!("{}", result);
}