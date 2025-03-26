use std::io::{self, Read};

struct Solution;

impl Solution {
    pub fn max_score(a: Vec<i32>, b: Vec<i32>) -> i64 {
        let mut f = [0i64; 5];
        for i in 1..5 {
            f[i] = i64::MIN / 2;
        }
        
        for y in b {
            let y = y as i64;
            for j in (0..=3).rev() {
                let a_j = a[j] as i64;
                let current = f[j] + a_j * y;
                if current > f[j + 1] {
                    f[j + 1] = current;
                }
            }
        }
        
        f[4]
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    
    let a_size: usize = tokens.next().unwrap().parse().unwrap();
    let b_size: usize = tokens.next().unwrap().parse().unwrap();
    
    let mut a = Vec::with_capacity(a_size);
    for _ in 0..a_size {
        a.push(tokens.next().unwrap().parse().unwrap());
    }
    
    let mut b = Vec::with_capacity(b_size);
    for _ in 0..b_size {
        b.push(tokens.next().unwrap().parse().unwrap());
    }
    
    println!("{}", Solution::max_score(a, b));
}