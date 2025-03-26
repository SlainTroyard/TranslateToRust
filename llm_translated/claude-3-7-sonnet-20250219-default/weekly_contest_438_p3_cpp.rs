use std::io;

fn exgcd(a: i32, b: i32, x: &mut i32, y: &mut i32) {
    if b == 0 {
        *x = 1;
        *y = 0;
        return;
    }
    exgcd(b, a % b, y, x);
    *y -= a / b * *x;
}

struct Solution;

impl Solution {
    pub fn has_same_digits(s: String) -> bool {
        let n = s.len();
        let s_bytes = s.as_bytes();

        // Precompute powers of 2 and 5 modulo 10
        let mut p2 = vec![1; n + 1];
        let mut p5 = vec![1; n + 1];
        for i in 1..=n {
            p2[i] = (p2[i - 1] * 2) % 10;
            p5[i] = (p5[i - 1] * 5) % 10;
        }

        // Calculate the factorial representation
        let calc = |l: usize, r: usize| -> i32 {
            let len = r - l;
            let mut c = 1;
            let mut two = 0;
            let mut five = 0;
            let mut sum = 0;
            
            for (i, j) in (l..=r).zip(0..) {
                sum = (sum + ((s_bytes[i] - b'0') as i32 * p2[two] * p5[five] * c) % 10) % 10;
                
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
                exgcd(t as i32, 10, &mut x, &mut y);
                c = (c * ((x % 10 + 10) % 10)) % 10;
            }
            
            sum
        };

        calc(0, n - 2) == calc(1, n - 1)
    }
}

fn main() {
    // Read input string
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read input");
    let s = s.trim().to_string();
    
    // Solve the problem
    let sol = Solution::has_same_digits(s);
    
    // Output the result
    println!("{}", if sol { 1 } else { 0 });
}