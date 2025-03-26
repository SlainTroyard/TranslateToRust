use std::io;

struct Solution;

impl Solution {
    fn kth_character(mut k: i64, operations: &[i32]) -> char {
        k -= 1;
        let mut inc = 0;
        
        // Find the position of the most significant bit (equivalent to __lg in C++)
        let highest_bit = 63 - k.leading_zeros() as i32;
        
        for i in (0..=highest_bit).rev() {
            if (k >> i) & 1 == 1 {
                inc += operations[i as usize];
            }
        }
        
        // Convert to character ('a' + inc % 26)
        (b'a' + (inc % 26) as u8) as char
    }
}

fn main() {
    // Read k and operation_size from the first line
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.trim().split_whitespace();
    
    let k: i64 = iter.next().unwrap().parse().expect("Expected valid k");
    let operation_size: usize = iter.next().unwrap().parse().expect("Expected valid operation_size");
    
    // Read operations from the second line
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let mut operations = vec![0; operation_size];
    let values: Vec<i32> = input.trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Expected valid integer"))
        .collect();
    
    for i in 0..operation_size {
        operations[i] = values[i];
    }
    
    // Solve and print result
    let s = Solution;
    println!("{}", s.kth_character(k, &operations));
}