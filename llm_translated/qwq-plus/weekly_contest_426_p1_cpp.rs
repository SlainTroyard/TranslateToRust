use std::io;

struct Solution;

impl Solution {
    fn smallest_number(n: u32) -> u32 {
        let b = 32 - n.leading_zeros();
        (1 << b) - 1
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n: u32 = input.trim().parse().expect("Invalid input");
    let result = Solution::smallest_number(n);
    println!("{}", result);
}