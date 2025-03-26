// Problem: Weekly Contest 421 Problem 2

use std::io;

fn length_after_transformations(s: &str, t: usize) -> i64 {
    const MOD: i64 = 1_000_000_007;
    let mut cnt = [0; 26];

    // Count the frequency of each character in the string
    for c in s.chars() {
        let index = (c as u8 - b'a') as usize;
        cnt[index] += 1;
    }

    // Perform the transformations
    for _ in 0..t {
        let mut nxt = [0; 26];
        for j in 0..25 {
            nxt[j + 1] = cnt[j];
        }
        nxt[0] = (nxt[0] + cnt[25]) % MOD;
        nxt[1] = (nxt[1] + cnt[25]) % MOD;
        cnt = nxt;
    }

    // Calculate the final result
    let mut ans: i64 = 0;
    for &count in &cnt {
        ans = (ans + count) % MOD;
    }
    ans
}

fn main() {
    // Read input from stdin
    let mut s = String::new();
    let mut t_str = String::new();

    io::stdin().read_line(&mut s).expect("Failed to read string");
    io::stdin().read_line(&mut t_str).expect("Failed to read integer");

    let s = s.trim(); // Trim whitespace
    let t: usize = t_str.trim().parse().expect("Failed to parse integer");

    // Create a Solution instance and call the function
    let result = length_after_transformations(s, t);

    // Write the result to stdout
    println!("{}", result);
}