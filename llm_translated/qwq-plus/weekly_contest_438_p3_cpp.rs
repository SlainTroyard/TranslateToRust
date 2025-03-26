struct Solution;

impl Solution {
    fn exgcd(a: i32, b: i32) -> (i32, i32) {
        if b == 0 {
            (1, 0)
        } else {
            let (y, x) = Self::exgcd(b, a % b);
            let quotient = a / b;
            let new_y = y - quotient * x;
            (x, new_y)
        }
    }

    fn has_same_digits(s: &str) -> bool {
        let n = s.len();
        if n < 3 {
            return false;
        }

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
            let mut i = l;
            let mut j = 0;

            while i < r {
                let digit = s.chars().nth(i).unwrap().to_digit(10).unwrap() as i32;
                let term = digit * p2[two] * p5[five] * c;
                sum = (sum + term) % 10;

                if i == r - 1 {
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

                let mut t_j = j + 1;
                while t_j % 2 == 0 {
                    two -= 1;
                    t_j /= 2;
                }
                while t_j % 5 == 0 {
                    five -= 1;
                    t_j /= 5;
                }

                let (x, y) = Self::exgcd(t_j as i32, 10);
                let inv_x = (x % 10 + 10) % 10;
                c = (c * inv_x) % 10;

                i += 1;
                j += 1;
            }
            sum
        };

        let res1 = calc(0, n - 2);
        let res2 = calc(1, n - 1);
        res1 == res2
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    let s = input.trim();
    let result = Solution::has_same_digits(s);
    println!("{}", if result { 1 } else { 0 });
}