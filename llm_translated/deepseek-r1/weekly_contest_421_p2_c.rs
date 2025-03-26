use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;

fn length_after_transformations(s: &str, t: i32) -> i32 {
    let mut lst = [0i64; 26];
    
    // Count initial character frequencies
    for c in s.chars() {
        let idx = (c as u8 - b'a') as usize;
        lst[idx] += 1;
    }
    
    // Apply transformations
    for _ in 0..t {
        let z = lst[25];
        // Shift elements from index 1..24 to 2..25
        for i in (2..=25).rev() {
            lst[i] = lst[i - 1];
        }
        // Update first two positions
        lst[1] = (lst[0] + z) % MOD;
        lst[0] = z;
    }
    
    // Calculate total length modulo MOD
    let mut ans = 0;
    for &count in lst.iter() {
        ans = (ans + count) % MOD;
    }
    ans as i32
}

fn main() {
    // Read entire input and split into whitespace-separated tokens
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut parts = input.split_whitespace();
    
    // Parse input exactly as original C code's scanf
    let s = parts.next().expect("missing string input");
    let t: i32 = parts.next()
        .expect("missing t input")
        .parse()
        .expect("t must be an integer");
    
    // Compute and print result
    let result = length_after_transformations(s, t);
    println!("{}", result);
}