// Problem: Weekly Contest 438 Problem 3 translated to Rust

use std::io::{self, Write};

struct Solution;

impl Solution {
    // Extended Euclidean Algorithm to solve ax + by = gcd(a, b)
    fn exgcd(a: i32, b: i32, x: &mut i32, y: &mut i32) {
        if b == 0 {
            *x = 1;
            *y = 0;
            return;
        }
        let mut x1 = 0;
        let mut y1 = 0;
        Self::exgcd(b, a % b, &mut x1, &mut y1);
        *x = y1;
        *y = x1 - (a / b) * y1;
    }

    fn has_same_digits(s: &str) -> bool {
        let n = s.len();
        let s = s.as_bytes(); // Convert string to bytes for easier indexing

        // Precompute powers of 2 and 5 modulo 10
        let mut p2 = vec![1; n + 1];
        let mut p5 = vec![1; n + 1];
        for i in 1..=n {
            p2[i] = (p2[i - 1] * 2) % 10;
            p5[i] = (p5[i - 1] * 5) % 10;
        }

        // Closure to calculate the weighted sum modulo 10
        let calc = |l: usize, r: usize| -> i32 {
            let len = r - l;
            let mut c = 1; // Coefficient multiplier
            let mut two = 0; // Tracks number of factors of 2
            let mut five = 0; // Tracks number of factors of 5
            let mut sum = 0;

            let mut i = l;
            let mut j = 0;
            loop {
                sum = (sum + (s[i] - b'0') as i32 * p2[two] * p5[five] * c) % 10;

                if i == r {
                    break;
                }

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

                t = j + 1;
                while t % 2 == 0 {
                    two -= 1;
                    t /= 2;
                }
                while t % 5 == 0 {
                    five -= 1;
                    t /= 5;
                }
                let mut x = 0;
                let mut y = 0;
                Solution::exgcd(t as i32, 10, &mut x, &mut y);
                c = (c * ((x % 10 + 10) % 10)) % 10;

                i += 1;
                j += 1;
            }

            sum
        };

        calc(0, n - 2) == calc(1, n - 1)
    }
}

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let s = input.trim();

    // Solution
    let result = Solution::has_same_digits(s);

    // Output result (true -> 1, false -> 0 for compatibility with the CPP version)
    println!("{}", if result { 1 } else { 0 });
}