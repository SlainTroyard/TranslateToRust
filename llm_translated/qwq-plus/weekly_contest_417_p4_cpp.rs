use std::io;

struct Solution;

impl Solution {
    pub fn kth_character(k: u64, operations: &[i32]) -> char {
        let mut k = k;
        k -= 1; // Adjust to 0-based index
        let mut inc = 0;
        let max_bit = if k == 0 { 0 } else { k.ilog2() as usize };
        for i in (0..=max_bit).rev() {
            if (k & (1 << i)) != 0 {
                if i < operations.len() {
                    inc += operations[i] as i32;
                }
            }
        }
        let remainder = inc.rem_euclid(26) as u8;
        (b'a' + remainder) as char
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut tokens = input.split_whitespace();

    let k: u64 = tokens.next().expect("No k").parse().expect("Invalid k");
    let operation_size: usize = tokens.next().expect("No operation size").parse().expect("Invalid operation size");

    let mut operations = Vec::with_capacity(operation_size);
    for _ in 0..operation_size {
        let num = tokens.next().expect("No operation").parse().expect("Invalid operation");
        operations.push(num);
    }

    let s = Solution;
    let result = s.kth_character(k, &operations);
    println!("{}", result);
}