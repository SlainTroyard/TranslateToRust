use std::io::{self, Read};

/// A direct Rust translation of the given C++ solution.
/// Preserves the same algorithm and I/O behavior.
struct Solution;

impl Solution {
    /// Translated method from C++:
    /// long long countSubstrings(string s)
    ///
    /// Counts how many substrings (based on specific conditions)
    /// in the input string match the problem's criteria.
    fn count_substrings(&self, s: &str) -> i64 {
        let mut ans = 0i64;
        // `f` corresponds to the C++ array<array<int, 9>, 10> f{};
        // Using i64 to store counts to be safe with potentially large sums.
        let mut f = [[0i64; 9]; 10];

        for c in s.chars() {
            // Convert character '0'..'9' to the corresponding integer value 0..9
            let d = (c as u8 - b'0') as usize;

            // For each possible modulus m in [1..9]
            for m in 1..10 {
                let mut nf = [0i64; 9];
                // Start a new count array for the current substring extension
                nf[d % m] = 1;

                // Update counts based on existing remainders
                for rem in 0..m {
                    let idx = ((rem * 10) + d) % m;
                    nf[idx] += f[m][rem];
                }
                f[m] = nf;
            }
            // Accumulate answer from the row corresponding to digit d, remainder 0
            ans += f[d][0];
        }

        ans
    }
}

/// Reads all input from stdin, splits by whitespace, and returns the first token (or an empty string if none).
/// Matches the behavior of "cin >> s" in C++ which reads the first whitespace-delimited token.
fn get_first_token() -> String {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read from stdin");
    let mut tokens = input.split_whitespace();
    tokens.next().unwrap_or("").to_string()
}

fn main() {
    // In the original C++ code, we read a single string from stdin and output the result.
    let s = get_first_token();
    let solution = Solution;
    let ans = solution.count_substrings(&s);
    // Prints the result followed by a newline, just like C++'s "cout << ans << endl;"
    println!("{}", ans);
}