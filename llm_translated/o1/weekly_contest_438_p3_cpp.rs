// Weekly Contest 438 Problem 3 in Rust
// -----------------------------------
// This program reads a single string from stdin, checks the condition defined
// by has_same_digits, and prints 1 or 0 accordingly.

use std::io;

// A struct to mirror the C++ 'Solution' class
struct Solution;

impl Solution {
    // Extended Euclidean Algorithm (exgcd) - replicated from C++
    // Finds x, y such that a*x + b*y = gcd(a, b)
    fn exgcd(a: i32, b: i32, x: &mut i32, y: &mut i32) {
        if b == 0 {
            *x = 1;
            *y = 0;
            return;
        }
        Solution::exgcd(b, a % b, y, x);
        *y -= (a / b) * *x;
    }

    // Translated from the C++ hasSameDigits function
    fn has_same_digits(&self, s: &str) -> bool {
        let n = s.len();
        
        // Prepare powers mod 10
        let mut p2 = vec![0; n + 1];
        let mut p5 = vec![0; n + 1];
        p2[0] = 1;
        p5[0] = 1;

        for i in 1..=n {
            p2[i] = (p2[i - 1] * 2) % 10;
            p5[i] = (p5[i - 1] * 5) % 10;
        }

        // A direct translation of the C++ lambda calc
        let calc = |l: usize, r: usize| -> i32 {
            let len = (r as i32) - (l as i32);
            let mut c = 1;
            let mut two = 0;
            let mut five = 0;
            let mut sum = 0;

            let mut i = l;
            let mut j = 0;
            loop {
                // Equivalent to (s[i] - '0')
                let digit = (s.as_bytes()[i] - b'0') as i32;
                // Multiply by  P2[two], P5[five] and accumulated coefficient c
                sum = (sum + digit * p2[two as usize] * p5[five as usize] * c) % 10;

                // Break condition exactly as in the C++ code
                if i == r {
                    break;
                }

                // Update for next iteration
                let mut t = len - j;
                // Factor out powers of 2
                while t % 2 == 0 && t != 0 {
                    two += 1;
                    t /= 2;
                }
                // Factor out powers of 5
                while t % 5 == 0 && t != 0 {
                    five += 1;
                    t /= 5;
                }
                c = (c * (t % 10)) % 10; // multiply c by remaining t (mod 10)

                // Now remove powers from j+1
                t = j + 1;
                while t % 2 == 0 && t != 0 {
                    two -= 1;
                    t /= 2;
                }
                while t % 5 == 0 && t != 0 {
                    five -= 1;
                    t /= 5;
                }

                // Invert (j+1) mod 10 using exgcd
                let mut x = 0;
                let mut y = 0;
                Solution::exgcd(t, 10, &mut x, &mut y);
                // x % 10 could be negative in some languages, so we add 10 for safety
                c = (c * ((x % 10 + 10) % 10)) % 10;

                i += 1;
                j += 1;
            }
            sum
        };

        // Compare calc(0, n-2) with calc(1, n-1)
        calc(0, n - 2) == calc(1, n - 1)
    }
}

fn main() {
    // Read a single line from stdin (same as C++ "cin >> s;")
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let s = input.trim();

    // Create a Solution instance and compute result
    let sol = Solution;
    let result = sol.has_same_digits(s);

    // Print 1 if true, 0 if false (mirroring the C++ output format)
    println!("{}", if result { 1 } else { 0 });
}