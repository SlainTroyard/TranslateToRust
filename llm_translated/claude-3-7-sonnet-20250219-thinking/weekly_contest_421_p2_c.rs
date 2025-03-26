use std::io::{self, Read};

/// Calculates the length of string after applying transformation t times
/// Exact implementation of the LeetCode Weekly Contest 421 Problem 2
fn length_after_transformations(s: &str, t: i32) -> i32 {
    const MOD: i32 = 1_000_000_007;
    let mut lst = [0; 26];
    
    // Count frequency of each lowercase letter
    for ch in s.bytes() {
        lst[(ch - b'a') as usize] += 1;
    }
    
    // Perform t transformations
    let mut t_remaining = t;
    while t_remaining > 0 {
        let z = lst[25];
        for i in (2..=25).rev() {
            lst[i] = lst[i - 1];
        }
        lst[1] = (lst[0] + z) % MOD;
        lst[0] = z;
        t_remaining -= 1;
    }
    
    // Sum up all elements
    let mut ans = 0;
    for &count in &lst {
        ans = (ans + count) % MOD;
    }
    
    ans
}

fn main() {
    // Read string input
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read input");
    let s = s.trim();
    
    // Read integer t
    let mut t_input = String::new();
    io::stdin().read_line(&mut t_input).expect("Failed to read input");
    let t = t_input.trim().parse::<i32>().expect("Failed to parse t as integer");
    
    // Calculate and print result
    let result = length_after_transformations(s, t);
    print!("{}", result);
}