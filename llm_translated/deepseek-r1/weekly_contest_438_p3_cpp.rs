struct Solution;

impl Solution {
    fn exgcd(a: i32, b: i32) -> (i32, i32) {
        if b == 0 {
            (1, 0)
        } else {
            let (x, y) = Self::exgcd(b, a % b);
            (y, x - (a / b) * y)
        }
    }

    fn has_same_digits(s: String) -> bool {
        let s_chars: Vec<char> = s.chars().collect();
        let n = s_chars.len();

        let mut p2 = vec![1; n + 1];
        let mut p5 = vec![1; n + 1];
        for i in 1..=n {
            p2[i] = (p2[i - 1] * 2) % 10;
            p5[i] = (p5[i - 1] * 5) % 10;
        }

        let calc = |l: usize, r: usize| -> i32 {
            let len = r - l;
            let mut two = 0;
            let mut five = 0;
            let mut c = 1;
            let mut sum = 0;
            let mut i = l;
            let mut j = 0;

            loop {
                let digit = s_chars[i].to_digit(10).unwrap() as i32;
                sum = (sum + digit * p2[two] * p5[five] * c) % 10;

                if i == r {
                    break;
                }

                // Handle remaining terms (len - j)
                let mut t = (len - j) as i32;
                while t % 2 == 0 {
                    two += 1;
                    t /= 2;
                }
                while t % 5 == 0 {
                    five += 1;
                    t /= 5;
                }
                c = (c * t) % 10;

                // Handle current term adjustment (j + 1)
                let mut t2 = (j + 1) as i32;
                while t2 % 2 == 0 {
                    two -= 1;
                    t2 /= 2;
                }
                while t2 % 5 == 0 {
                    five -= 1;
                    t2 /= 5;
                }

                // Modular inverse for coprime part
                let (x, _) = Self::exgcd(t2, 10);
                let inv = (x % 10 + 10) % 10;
                c = (c * inv) % 10;

                i += 1;
                j += 1;
            }

            sum
        };

        calc(0, n - 2) == calc(1, n - 1)
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let s = input.trim().to_string();
    println!("{}", if Solution::has_same_digits(s) { 1 } else { 0 });
}