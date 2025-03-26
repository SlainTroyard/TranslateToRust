// Problem: Weekly Contest 413 Problem 1
use std::io;

struct Solution {}

impl Solution {
    pub fn check_two_chessboards(coordinate1: String, coordinate2: String) -> bool {
        let c1: Vec<char> = coordinate1.chars().collect();
        let c2: Vec<char> = coordinate2.chars().collect();
        ((c1[0] as i32 - c2[0] as i32) + (c1[1] as i32 - c2[1] as i32)) % 2 == 0
    }
}

fn main() {
    let mut coordinate1 = String::new();
    io::stdin()
        .read_line(&mut coordinate1)
        .expect("Failed to read line");
    coordinate1 = coordinate1.trim().to_string();

    let mut coordinate2 = String::new();
    io::stdin()
        .read_line(&mut coordinate2)
        .expect("Failed to read line");
    coordinate2 = coordinate2.trim().to_string();

    let sol = Solution {};
    if sol.check_two_chessboards(coordinate1, coordinate2) {
        println!("true");
    } else {
        println!("false");
    }
}