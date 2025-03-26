// Define a Solution struct to mirror the original C++ class structure
struct Solution;

impl Solution {
    /// Calculates the smallest number as per the problem's requirements.
    /// The algorithm computes the number of bits required to represent `n` and returns (2^b - 1).
    fn smallest_number(n: i32) -> i32 {
        // Convert to unsigned for bitwise logarithm calculation
        let n_unsigned = n as u32;
        // Calculate the number of bits needed. ilog2 returns floor(log2(n)), add 1 for bit count
        let b = n_unsigned.ilog2() + 1;
        // Compute 2^b - 1 using bit shifting
        (1 << b) - 1
    }
}

fn main() {
    // Read input from stdin
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    // Parse input to i32, panicking on invalid input
    let n = input.trim().parse().expect("Invalid integer input");
    
    // Compute the result using the same logic as the original code
    let result = Solution::smallest_number(n);
    
    // Output the result with a trailing newline
    println!("{}", result);
}