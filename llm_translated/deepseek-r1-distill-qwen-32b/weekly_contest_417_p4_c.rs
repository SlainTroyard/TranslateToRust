use std::io;

struct Solution {}

impl Solution {
    /// Finds the k-th character based on the given operations.
    /// The character is determined by summing specific operations based on the binary representation of k.
    fn kth_character(k: u64, operations: Vec<i32>) -> char {
        let mut k = k;
        k -= 1;
        let mut inc = 0;
        let max_bit = (k).bit_length().saturating_sub(1);
        for i in (0..=max_bit).rev() {
            if (k >> i) & 1 != 0 {
                inc += operations[i as usize];
            }
        }
        let c = ('a' as u8 + (inc % 26) as u8) as char;
        c
    }
}

fn main() {
    // Read the first line containing k and operationSize
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    let k = parts[0].parse::<u64>().expect("Invalid k");
    let operation_size = parts[1].parse::<usize>().expect("Invalid operation size");
    
    // Read the operations vector
    let mut operations = Vec::with_capacity(operation_size);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read operations");
    let op_parts: Vec<&str> = input.trim().split_whitespace().collect();
    for i in 0..operation_size {
        let val = op_parts[i].parse::<i32>().expect("Invalid operation");
        operations.push(val);
    }
    
    // Compute and print the result
    let solution = Solution {};
    let result = solution.kth_character(k, operations);
    println!("{}", result);
}