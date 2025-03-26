use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn kth_character(mut k: i64, operations: &Vec<i32>) -> char {
        k -= 1;
        let mut inc = 0;
        
        // Equivalent to C++'s __lg(k) which finds the floor of log2(k)
        let highest_bit = 63 - k.leading_zeros() as i32;
        
        for i in (0..=highest_bit).rev() {
            if (k >> i) & 1 == 1 {
                inc += operations[i as usize];
            }
        }
        
        // Convert to char, ensuring we wrap around the alphabet (a-z)
        (b'a' + (inc % 26) as u8) as char
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read k and operationSize
    let input_line = lines.next().unwrap()?;
    let mut parts = input_line.split_whitespace();
    let k: i64 = parts.next().unwrap().parse().unwrap();
    let operation_size: usize = parts.next().unwrap().parse().unwrap();
    
    // Read operations
    let input_line = lines.next().unwrap()?;
    let operations: Vec<i32> = input_line
        .split_whitespace()
        .take(operation_size)
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Call the solution and print the result
    let s = Solution;
    println!("{}", s.kth_character(k, &operations));
    
    Ok(())
}