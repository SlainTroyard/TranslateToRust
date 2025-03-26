use std::io::{self, Write};

struct Solution;

impl Solution {
    // Extended Euclidean Algorithm to find the modular inverse
    fn exgcd(a: i32, b: i32) -> (i32, i32) {
        if b == 0 {
            (1, 0)
        } else {
            let (x, y) = Solution::exgcd(b, a % b);
            (y, x - (a / b) * y)
        }
    }

    // Function to check if the digits of the string have the same sum
    fn has_same_digits(s: &str) -> bool {
        let n = s.len();

        // Precompute powers of 2 and 5 modulo 10
        let mut p2 = vec![1; n + 1];
        let mut p5 = vec![1; n + 1];
        for i in 1..=n {
            p2[i] = (p2[i - 1] * 2) % 10;
            p5[i] = (p5[i - 1] * 5) % 10;
        }

        // Lambda to calculate the sum of digits in a given range
        let calc = |l: usize, r: usize| -> i32 {
            let len = r - l;
            let mut c = 1;
            let mut two = 0;
            let mut five = 0;
            let mut sum = 0;
            let mut i = l;
            let mut j = 0;

            loop {
                let digit = s.chars().nth(i).unwrap().to_digit(10).unwrap() as i32;
                sum = (sum + digit * p2[two] * p5[five] * c) % 10;
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
                let (x, _) = Solution::exgcd(t, 10);
                c = (c * (x % 10 + 10)) % 10;
                i += 1;
                j += 1;
            }
            sum
        };

        // Compare the sums of the two ranges
        calc(0, n - 2) == calc(1, n - 1)
    }
}

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let s = input.trim();

    // Check if the digits have the same sum
    let result = Solution::has_same_digits(s);

    // Print the result
    println!("{}", result);
}