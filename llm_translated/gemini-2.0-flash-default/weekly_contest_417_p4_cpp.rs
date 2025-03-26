use std::io;

struct Solution {}

impl Solution {
    pub fn kth_character(k: i64, operations: &Vec<i32>) -> char {
        let k = k - 1;
        let mut inc = 0;
        let len = operations.len();
        for i in (0..len).rev() {
            if (k >> i) & 1 == 1 {
                inc += operations[i];
            }
        }
        ((b'a' + (inc % 26) as u8) as char)
    }
}

fn main() {
    let mut k = String::new();
    io::stdin().read_line(&mut k).expect("Failed to read line");
    let mut k = k.trim().split_whitespace();
    let k_val: i64 = k.next().unwrap().parse().expect("Not a number");
    let operation_size: usize = k.next().unwrap().parse().expect("Not a number");

    let mut operations_str = String::new();
    io::stdin().read_line(&mut operations_str).expect("Failed to read line");
    let operations: Vec<i32> = operations_str
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();

    let s = Solution {};
    println!("{}", s.kth_character(k_val, &operations));
}