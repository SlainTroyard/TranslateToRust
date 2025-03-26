use std::io;

/// A struct representing our solution, mimicking the C++ class structure
struct Solution;

impl Solution {
    /// Calculate the smallest number for a given n.
    /// This computes the number of bits b of n (as floor(log2(n)) + 1),
    /// then returns 2^b - 1 by shifting 1 left by b bits.
    fn smallest_number(&self, n: i32) -> i32 {
        let b = ((n as f64).log2().floor() as i32) + 1;
        (1 << b) - 1
    }
}

fn main() {
    // Read a single integer n from stdin (mimicking 'cin >> n;')
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line from stdin");
    let n: i32 = buffer.trim().parse().expect("Failed to parse input as integer");

    // Instantiate the solution object
    let solution = Solution;

    // Compute the smallest number
    let result = solution.smallest_number(n);

    // Output the result (mimicking 'cout << result << endl;')
    println!("{}", result);
}