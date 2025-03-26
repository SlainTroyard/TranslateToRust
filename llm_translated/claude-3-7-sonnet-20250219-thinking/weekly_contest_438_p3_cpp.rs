use std::io;

struct Solution;

impl Solution {
    fn exgcd(a: i32, b: i32, x: &mut i32, y: &mut i32) {
        if b == 0 {
            *x = 1;
            *y = 0;
            return;
        }
        Solution::exgcd(b, a % b, y, x);
        *y -= a / b * *x;
    }

    fn has_same_digits(s: &str) -> bool {
        let n = s.len();
        
        let mut p2 = vec![0; n + 1];
        let mut p5 = vec![0; n + 1];
        p2[0] = 1;
        p5[0] = 1;
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
            
            let bytes = s.as_bytes();
            
            let mut i = l;
            let mut j = 0;
            loop {
                sum = (sum + ((bytes[i] - b'0') as i32 * p2[two] * p5[five] * c) % 10) % 10;
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
                
                let mut x = 0;
                let mut y = 0;
                Solution::exgcd(t as i32, 10, &mut x, &mut y);
                c = (c * (x % 10 + 10)) % 10;
                
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
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let s = input.trim();
    
    let result = Solution::has_same_digits(s);
    println!("{}", if result { 1 } else { 0 });
}