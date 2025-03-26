fn main() {
    use std::io;

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("Failed to read line");
    let n: i32 = input_line.trim().parse().expect("Invalid input");

    let result = Solution::smallest_number(n);
    println!("{}", result);
}

struct Solution {}

impl Solution {
    pub fn smallest_number(n: i32) -> i32 {
        let b = (n as f64).log2().ceil() as u32; // Calculate the number of bits, using ceil to match C++ behavior of log2(n) + 1 for integer cast.
        (1 << b) - 1 // Return 2^b - 1
    }
}