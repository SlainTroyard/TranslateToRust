use std::io::{self, Read};
use std::cmp::min;
use std::iter::repeat;
use std::collections::HashMap as HH ;
// Constants used in the solution
const MOD: i64 = 1_000_000_007;
const SIZE: usize = 26;
type Matrix = [[i64; SIZE]; SIZE];

struct Solution;

impl Solution {
    // Returns the result of matrix multiplication `a * b`
    fn mul(a: &Matrix, b: &Matrix) -> Matrix {
        let mut c = [[0; SIZE]; SIZE];
        for i in 0..SIZE {
            for k in 0..SIZE {
                if a[i][k] == 0 {
                    continue;
                }
                for j in 0..SIZE {
                    c[i][j] = (c[i][j] + a[i][k] * b[k][j] % MOD) % MOD;
                }
            }
        }
        c
    }

    // Returns the matrix `a` raised to the power `n`
    fn pow(mut a: Matrix, mut n: i32) -> Matrix {
        let mut res = [[0; SIZE]; SIZE];
        for i in 0..SIZE {
            res[i][i] = 1; // Initialize to identity matrix
        }
        while n > 0 {
            if n & 1 == 1 {
                res = Self::mul(&res, &a);
            }
            a = Self::mul(&a, &a);
            n >>= 1;
        }
        res
    }

    pub fn length_after_transformations(
        s: &str,
        t: i32,
        nums: &[i32],
    ) -> i64 {
        let mut m = [[0; SIZE]; SIZE];
        for i in 0..SIZE {
            for j in (i + 1)..=(i + nums[i] as usize) {
                m[i][j % SIZE] = 1;
            }
        }
        m = Self::pow(m, t);

        let mut cnt = [0; SIZE];
        for c in s.chars() {
            cnt[(c as u8 - b'a') as usize] += 1;
        }

        let mut ans = 0;
        for i in 0..SIZE {
            let sum: i64 = m[i].iter().sum();
            ans = (ans + sum * cnt[i] as i64 % MOD) % MOD;
        }
        ans
    }
}

fn main() {
    // Parse stdin to extract inputs
    let mut stdin = String::new();
    io::stdin().read_to_string(&mut stdin).unwrap();
    let mut lines = stdin.lines();
    
    let s_len: usize = lines.next().unwrap().trim().parse().unwrap();
    let s = lines.next().unwrap().trim().to_string();
    let t: i32 = lines.next().unwrap().trim().parse().unwrap();
    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Make sure input met expectations
    assert_eq!(s.len(), s_len);
    assert_eq!(nums.len(), SIZE);

    let solution = Solution;
    let ans = solution.length_after_transformations(&s, t, &nums);

    // Print the output to stdout
    println!("{}", ans);
}