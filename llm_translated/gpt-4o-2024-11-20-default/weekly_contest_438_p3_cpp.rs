// Problem: Weekly Contest 438 Problem 3
use std::io::{self, Write};

struct Solution;

impl Solution {
    fn exgcd(a: i32, b: i32, x: &mut i32, y: &mut i32) {
        if b == 0 {
            *x = 1;
            *y = 0;
            return;
        }
        let mut temp_x = 0;
        let mut temp_y = 0;
        Self::exgcd(b, a % b, &mut temp_x, &mut temp_y);
        *x = temp_y;
        *y = temp_x - (a / b) * temp_y;
    }

    fn has_same_digits(s: &str) -> bool {
        let n = s.len();

        let mut p2 = vec![1; n + 1];
        let mut p5 = vec![1; n + 1];
        for i in 1..=n {
            p2[i] = (p2[i - 1] * 2) % 10;
            p5[i] = (p5[i - 1] * 5) % 10;
        }

        let calc = |l: usize, r: usize| -> i32 {
            let len = r - l;
            let mut c = 1;
            let mut two = 0;
            let mut five = 0;
            let mut sum = 0;
            for (i, j) in (l..).zip(0..) {
                sum = (sum + (s.as_bytes()[i] - b'0') as i32 * p2[two] * p5[five] * c) % 10;
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
                Self::exgcd(t, 10, &mut x, &mut y);
                c = (c * ((x % 10 + 10) % 10)) % 10;
            }
            sum
        };

        calc(0, n - 2) == calc(1, n - 1)
    }
}

fn main() {
    // Reading input from stdin and outputting result to stdout
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let s = input.trim();
    
    let sol = Solution;
    let result = sol.has_same_digits(s);

    // Output result
    println!("{}", if result { 1 } else { 0 });
}