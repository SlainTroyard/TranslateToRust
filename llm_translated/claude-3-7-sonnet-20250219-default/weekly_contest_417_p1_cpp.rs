use std::io;

struct Solution;

impl Solution {
    pub fn kth_character(k: i64) -> char {
        // Calculate the character by counting the number of set bits (1s) in k-1
        // and adding that to the ASCII value of 'a'
        let bits = (k - 1).count_ones() as u8;
        ('a' as u8 + bits) as char
    }
}

fn main() {
    // Read the input value k
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let k: i64 = input.trim().parse().expect("Input should be an integer");
    
    // Create solution instance and compute the result
    let s = Solution;
    println!("{}", s.kth_character(k));
}