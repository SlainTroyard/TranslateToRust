use std::io;

struct Solution;

impl Solution {
    // Check if the sum of the differences between the first characters and the second characters of the two coordinates is even.
    fn check_two_chessboards(coordinate1: &str, coordinate2: &str) -> bool {
        let c1 = coordinate1.as_bytes();
        let c2 = coordinate2.as_bytes();
        let diff = (c1[0] as i32 - c2[0] as i32) + (c1[1] as i32 - c2[1] as i32);
        diff % 2 == 0
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)
        .expect("Failed to read input"); // Panic on I/O error as per problem's implicit requirement

    let mut parts = input.split_whitespace();
    let coordinate1 = parts.next().expect("No first coordinate");
    let coordinate2 = parts.next().expect("No second coordinate");

    let result = Solution::check_two_chessboards(coordinate1, coordinate2);
    println!("{}", result);
}