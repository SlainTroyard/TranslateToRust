use std::io;

struct Solution;

impl Solution {
    // Function to check if the string contains a special substring of length `k`
    fn has_special_substring(s: &str, k: usize) -> bool {
        let n = s.len();
        let mut cnt = 0;
        let chars: Vec<char> = s.chars().collect();

        for i in 0..n {
            cnt += 1;
            if i == n - 1 || chars[i] != chars[i + 1] {
                if cnt == k {
                    return true;
                }
                cnt = 0;
            }
        }
        false
    }
}

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Split the input into the string and the integer k
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() != 2 {
        eprintln!("Invalid input format");
        return;
    }

    let s = parts[0];
    let k: usize = parts[1].parse().expect("Failed to parse k");

    // Check if the string contains a special substring of length k
    let result = Solution::has_special_substring(s, k);

    // Print the result
    println!("{}", result);
}