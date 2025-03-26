use std::io;

/// Counts the number of substrings that form a number divisible by any m (1 ≤ m ≤ 9).
///
/// # Arguments
///
/// * `s` - The input string of digits.
///
/// # Returns
///
/// The total count of valid substrings as a 64-bit integer.
fn count_substrings(s: &str) -> u64 {
    let mut ans: u64 = 0;
    // f[m][rem] represents the count of substrings ending at current position with remainder `rem` modulo m.
    // The first dimension has 10 elements (indices 0-9) for m from 0 to 9 (though m starts at 1).
    // The second dimension has 9 elements (indices 0-8) to accommodate the maximum m=9.
    let mut f = [[0usize; 9]; 10];

    for c in s.chars() {
        let d = (c as u8 - b'0') as usize;
        for m in 1..=9 {
            // Temporary array to store new counts for current m
            let mut nf = [0; 9];
            let rem_d = d % m;
            // Initialize with the single-digit case
            nf[rem_d as usize] = 1;
            // Update counts based on previous remainders
            for rem in 0..m {
                let new_rem = (rem * 10 + d) % m;
                nf[new_rem as usize] += f[m][rem as usize];
            }
            // Update f[m] with new counts
            for rem in 0..m {
                f[m][rem as usize] = nf[rem as usize];
            }
        }
        // Add the count for m = d (current digit)
        ans += f[d][0] as u64;
    }
    ans
}

fn main() {
    let mut input = String::new();
    if let Err(e) = io::stdin().read_line(&mut input) {
        eprintln!("Error reading input: {}", e);
        std::process::exit(1);
    }
    let trimmed = input.trim();
    let result = count_substrings(trimmed);
    println!("{}", result);
}