use std::cmp::max;
use std::io;

fn answer_string(s: String, k: i32) -> String {
    // If k is 1, return the original string as per the problem statement
    if k == 1 {
        return s;
    }
    let n = s.len();
    let mut ans = String::new(); // Initialize an empty string to store the answer

    // Iterate through the string from the beginning to the end
    for i in 0..n {
        // Calculate the length of the substring.
        // The length is determined by 'n - max(k - 1, i)' as in the original C++ code.
        let len = n - max(k - 1, i as i32) as usize;

        // Extract the substring from index 'i' with the calculated length.
        // In Rust, string slicing is done using ranges.
        // We need to ensure that the end index (i + len) is within the bounds of the string.
        // However, the original C++ code uses `substr` which handles out-of-bounds lengths gracefully.
        // In Rust, string slicing `&s[i..i + len]` would panic if `i + len > n`.
        // But based on the logic, `len` should always be non-negative and `i + len <= n` should hold within the loop condition `i < n` if the length calculation is correct.
        let sub = if len > 0 { // Ensure length is positive to avoid empty substring issues. Although in this problem, length should always be positive if i < n and k >= 1.
            &s[i..i + len]
        } else {
            "" // Should not happen given the problem constraints and logic, but as a safety measure.
        };

        // Compare the current substring 'sub' with the current answer 'ans' lexicographically.
        // In Rust, strings can be compared directly using operators like '>' for lexicographical comparison.
        if sub > &ans {
            ans = sub.to_string(); // If 'sub' is lexicographically greater, update 'ans'.
        }
    }
    ans // Return the final lexicographically largest substring found.
}

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    let s = s.trim(); // Remove trailing newline and whitespace

    let mut k_str = String::new();
    io::stdin().read_line(&mut k_str).expect("Failed to read line");
    let k: i32 = k_str.trim().parse().expect("Failed to parse k as i32"); // Parse k as integer

    let result = answer_string(s.to_string(), k); // Call the answerString function
    println!("{}", result); // Print the result to the console
}