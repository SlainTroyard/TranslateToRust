use std::collections::HashMap;
use std::io::{self, Read};

// Calculate the greatest common divisor of two numbers using their absolute values
fn gcd(a: i32, b: i32) -> i32 {
    let mut a = a.abs();
    let mut b = b.abs();
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

// Main algorithm implementation
fn number_of_subsequences(nums: &[i32]) -> i64 {
    let n = nums.len();
    let mut suf = HashMap::new();

    // Preprocess suffix pairs (c, d) where c is at index i (4 <= i < n-2)
    // and d is at index j (i+2 <= j < n)
    for i in 4..(n - 2) {
        let c = nums[i];
        for j in (i + 2)..n {
            let d = nums[j];
            let g = gcd(c, d);
            let d_div_g = d / g;
            let c_div_g = c / g;
            // Combine the reduced pair into a packed i32 key
            let key = (d_div_g << 16) | (c_div_g & 0xFFFF);
            *suf.entry(key).or_insert(0) += 1;
        }
    }

    let mut ans: i64 = 0;

    // Process each possible middle element b at index i (2 <= i < n-4)
    for i in 2..(n - 4) {
        let b = nums[i];
        
        // Count valid prefix pairs (a, b) where a is at index j (0 <= j < i-1)
        for j in 0..(i - 1) {
            let a = nums[j];
            let g = gcd(a, b);
            let a_div_g = a / g;
            let b_div_g = b / g;
            // Lookup matching suffix pairs in the preprocessed map
            let key = (a_div_g << 16) | (b_div_g & 0xFFFF);
            ans += *suf.get(&key).unwrap_or(&0) as i64;
        }

        // Remove processed suffix pairs where c is at i+2 and d is at j (i+4 <= j < n)
        let c = nums[i + 2];
        for j in (i + 4)..n {
            let d = nums[j];
            let g = gcd(c, d);
            let d_div_g = d / g;
            let c_div_g = c / g;
            let key = (d_div_g << 16) | (c_div_g & 0xFFFF);
            *suf.entry(key).or_insert(0) -= 1;
        }
    }

    ans
}

// Preserve exact I/O behavior from original C++ code
fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    // Read array size and elements
    let n: usize = tokens.next().unwrap().parse().unwrap();
    let nums: Vec<i32> = tokens.take(n).map(|s| s.parse().unwrap()).collect();

    // Compute and print result
    let result = number_of_subsequences(&nums);
    println!("{}", result);
}