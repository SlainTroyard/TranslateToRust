use std::io::{self, BufRead};

fn exgcd(a: i32, b: i32) -> (i32, i32) {
    if b == 0 {
        (1, 0)
    } else {
        let (x, y) = exgcd(b, a % b);
        (y, x - (a / b) * y)
    }
}

struct Solution;

impl Solution {
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
                let (x, _) = exgcd(t, 10);
                c = (c * (x % 10 + 10)) % 10;
            }
            sum
        };

        calc(0, n - 2) == calc(1, n - 1)
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    if let Some(Ok(line)) = lines.next() {
        let s = line.trim();
        let sol = Solution;
        println!("{}", if sol.has_same_digits(s) { "true" } else { "false" });
    } else {
        eprintln!("Error: No input provided");
    }
}