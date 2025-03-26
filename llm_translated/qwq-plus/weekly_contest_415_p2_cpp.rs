use std::io;

pub struct Solution;

impl Solution {
    pub fn max_score(a: Vec<i32>, b: Vec<i32>) -> i64 {
        let mut f = [0i64; 5];
        for i in 1..5 {
            f[i] = i64::MIN / 2;
        }

        for &y in &b {
            for j in (0..4).rev() {
                let current = f[j] + (a[j] as i64) * y as i64;
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
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut tokens = input.split_whitespace();

    let a_size: usize = tokens.next().expect("No aSize").parse().expect("Invalid aSize");
    let b_size: usize = tokens.next().expect("No bSize").parse().expect("Invalid bSize");

    let a: Vec<i32> = (0..a_size)
        .map(|_| tokens.next().expect("No a element").parse().expect("Invalid a element"))
        .collect();

    let b: Vec<i32> = (0..b_size)
        .map(|_| tokens.next().expect("No b element").parse().expect("Invalid b element"))
        .collect();

    let solution = Solution;
    println!("{}", solution.max_score(a, b));
}