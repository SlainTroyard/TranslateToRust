use std::io;

struct Solution {}

impl Solution {
    pub fn kth_character(k: i64, operations: &Vec<i32>) -> char {
        let k = k - 1;
        let mut inc = 0;
        for i in (0..64).rev() {
            if (k >> i) & 1 == 1 {
                if i < operations.len() as i32 {
                  inc += operations[i as usize];
                }
            }
        }
        return (b'a' + (inc % 26) as u8) as char;
    }
}

fn main() {
    let mut k = String::new();
    io::stdin().read_line(&mut k).unwrap();
    let mut iter = k.trim().split_whitespace();

    let k: i64 = iter.next().unwrap().parse().unwrap();
    let operation_size: usize = iter.next().unwrap().parse().unwrap();
    
    let mut operations_str = String::new();
    io::stdin().read_line(&mut operations_str).unwrap();
    let operations: Vec<i32> = operations_str
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let s = Solution {};
    println!("{}", s.kth_character(k, &operations));
}