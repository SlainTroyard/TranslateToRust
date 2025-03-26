use std::io::{self, BufRead};

struct Solution {}

impl Solution {
    pub fn kth_character(k: i64, operations: &Vec<i32>) -> char {
        let k = k - 1;
        let mut inc = 0;
        
        // Equivalent to C++'s __lg(k) - finding the most significant bit position
        let max_bit = 63 - k.leading_zeros() as i32;
        
        for i in (0..=max_bit).rev() {
            if (k >> i) & 1 == 1 {
                inc += operations[i as usize];
            }
        }
        
        // Compute the resulting character
        (b'a' + (inc % 26) as u8) as char
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Parse k and operationSize from the first line
    let input_line = lines.next().unwrap().unwrap();
    let mut parts = input_line.split_whitespace();
    let k: i64 = parts.next().unwrap().parse().unwrap();
    let operation_size: usize = parts.next().unwrap().parse().unwrap();
    
    // Parse the operations array from the second line
    let input_line = lines.next().unwrap().unwrap();
    let operations: Vec<i32> = input_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Call the solution and print the result
    let s = Solution {};
    println!("{}", s.kth_character(k, &operations));
}