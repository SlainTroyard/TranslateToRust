use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn exgcd(a: i32, b: i32) -> (i32, i32) {
        if b == 0 {
            (1, 0)
        } else {
            let (y, x) = Self::exgcd(b, a % b);
            (y, x - (a / b) * y)
        }
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
            for (i, j) in (l..=r).zip(0..) {
                sum = (sum + (s.chars().nth(i).unwrap() as i32 - '0' as i32) * p2[two] * p5[five] * c) % 10;
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
                let mut t = j + 1;
                while t % 2 == 0 {
                    two -= 1;
                    t /= 2;
                }
                while t % 5 == 0 {
                    five -= 1;
                    t /= 5;
                }
                let (x, _) = Self::exgcd(t as i32, 10);
                c = (c * ((x % 10 + 10) % 10)) % 10;
            }
            sum
        };

        calc(0, n - 2) == calc(1, n - 1)
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    while let Some(Ok(s)) = lines.next() {
        let result = Solution::has_same_digits(&s);
        println!("{}", result);
    }

    Ok(())
}