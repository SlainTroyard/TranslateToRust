use std::io::{self, BufRead, Write};

/// Compute the extended GCD of `a` and `b`.
/// This function sets `x` and `y` such that: a*x + b*y = gcd(a,b).
fn exgcd(a: i32, b: i32, x: &mut i32, y: &mut i32) {
    if b == 0 {
        *x = 1;
        *y = 0;
    } else {
        // Temporary holders for the results from the recursive call.
        let mut x1 = 0;
        let mut y1 = 0;
        // Recurse with (b, a % b), swapping the roles of x and y.
        exgcd(b, a % b, &mut x1, &mut y1);
        *x = y1;
        *y = x1 - (a / b) * y1;
    }
}

/// The struct Solution holds our methods.
struct Solution;

impl Solution {
    /// Given a string of digits `s`, check whether the two computed values (based on our complex digit operations)
    /// are equal.
    ///
    /// The algorithm follows the same logic as the provided C++ code.
    fn has_same_digits(s: &str) -> bool {
        let n = s.len();
        // Safety: the algorithm in C++ assumes that s has at least 2 digits.
        // Create power arrays for 2 and 5 modulo 10, with indices from 0 to n (inclusive).
        let mut p2 = vec![0; n + 1];
        let mut p5 = vec![0; n + 1];
        p2[0] = 1;
        p5[0] = 1;
        for i in 1..=n {
            p2[i] = (p2[i - 1] * 2) % 10;
            p5[i] = (p5[i - 1] * 5) % 10;
        }

        // Convert string to bytes for direct indexing and digit extraction.
        let s_bytes = s.as_bytes();

        // Define a closure that computes a value based on a subrange of the digits.
        // The closure mimics the lambda 'calc' in the original C++ code.
        let calc = |l: usize, r: usize| -> i32 {
            // len = r - l, note that the loop will run from index l to r (inclusive).
            let len = (r - l) as i32;
            let mut c = 1;
            let mut two = 0;
            let mut five = 0;
            let mut sum = 0;
            let mut j = 0;
            // Loop from index l to r (inclusive)
            for i in l..=r {
                // Calculate current digit value: s[i] - '0'
                let digit = (s_bytes[i] as char).to_digit(10).unwrap() as i32;
                // Update the sum modulo 10.
                sum = (sum + digit * p2[two as usize] * p5[five as usize] * c) % 10;
                // If we've reached the last index, exit the loop.
                if i == r {
                    break;
                }
                // Multiply current denominator component (t) and update counters for factors 2 and 5.
                let mut t = len - j;
                while t % 2 == 0 {
                    two += 1;
                    t /= 2;
                }
                while t % 5 == 0 {
                    five += 1;
                    t /= 5;
                }
                c = (c * t) % 10;
                // Remove factors from the numerator part.
                let mut t = j + 1;
                while t % 2 == 0 {
                    two -= 1;
                    t /= 2;
                }
                while t % 5 == 0 {
                    five -= 1;
                    t /= 5;
                }
                // Compute the modular inverse factor for t modulo 10 using the extended GCD.
                let mut x = 0;
                let mut y = 0;
                exgcd(t, 10, &mut x, &mut y);
                // Adjust x modulo 10 ensuring it's positive.
                let x_mod = ((x % 10) + 10) % 10;
                c = (c * x_mod) % 10;
                j += 1;
            }
            sum
        };

        // Compare the two calculated values as in the C++ code:
        // calc(0, n-2) and calc(1, n-1)
        // The result will be true if they are equal, false otherwise.
        calc(0, n - 2) == calc(1, n - 1)
    }
}

fn main() -> io::Result<()> {
    // Prepare to read the input from standard input.
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut input = String::new();
    // Read the entire input.
    reader.read_to_string(&mut input)?;
    
    // The C++ code uses "cin >> s" which reads a single token.
    // So we split by whitespace and take the first token.
    let token = input.split_whitespace().next().unwrap_or("");
    
    // Process the token using our solution method.
    let result = if Solution::has_same_digits(token) { 1 } else { 0 };
    
    // Write the result to standard output.
    let stdout = io::stdout();
    let mut writer = io::BufWriter::new(stdout.lock());
    writeln!(writer, "{}", result)?;
    Ok(())
}