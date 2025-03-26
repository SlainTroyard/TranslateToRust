use std::io::{self, Write};

fn number_of_substrings(s: &str, k: i32) -> i32 {
    let mut hash_arr = [0; 26];
    let mut left = 0;
    let mut right = 0;
    let s_len = s.len();
    let mut res = 0;
    let s_bytes = s.as_bytes();

    while left < s_len && right < s_len {
        let idx = (s_bytes[right] - b'a') as usize;
        hash_arr[idx] += 1;
        if hash_arr[idx] == k {
            res += (s_len - right) as i32;
            while left <= right {
                let left_idx = (s_bytes[left] - b'a') as usize;
                hash_arr[left_idx] -= 1;
                left += 1;
                if hash_arr[left_idx] != k - 1 {
                    res += (s_len - right) as i32;
                } else {
                    break;
                }
            }
        }
        right += 1;
    }

    res
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut parts = input.trim().split_whitespace();
    let s = parts.next().expect("No string input");
    let k: i32 = parts.next().expect("No k input").parse().expect("k is not a number");

    let result = number_of_substrings(s, k);
    println!("{}", result);
}