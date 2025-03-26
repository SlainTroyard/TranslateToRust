struct Solution;

impl Solution {
    // Returns the k-th character as per the problem's logic
    // The algorithm uses the number of set bits in (k-1) to determine the offset from 'a'
    fn kth_character(k: u64) -> char {
        let count = (k - 1).count_ones();
        // Convert 'a' to u8, add the bit count (as u8), then cast back to char
        (b'a' as u8 + count as u8) as char
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)
        .expect("Failed to read input"); // Panic on read error as per problem constraints
    let k: u64 = input.trim().parse()
        .expect("Input must be a valid non-negative integer"); // Panic on invalid input
    // Output the result with a newline, matching the original code's format
    println!("{}", Solution::kth_character(k));
}