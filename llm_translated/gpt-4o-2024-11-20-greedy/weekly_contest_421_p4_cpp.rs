use std::io::{self, Read};
use std::iter::repeat;
use std::cmp::min;

const MOD: i64 = 1_000_000_007;
const SIZE: usize = 26;

type Matrix = [[i64; SIZE]; SIZE];

struct Solution;

impl Solution {
    // Multiplies two matrices a and b
    fn mul(a: &Matrix, b: &Matrix) -> Matrix {
        let mut c = [[0; SIZE]; SIZE];
        for i in 0..SIZE {
            for k in 0..SIZE {
                if a[i][k] == 0 {
                    continue;
                }
                for j in 0..SIZE {
                    c[i][j] = (c[i][j] + a[i][k] * b[k][j]) % MOD;
                }
            }
        }
        c
    }

    // Computes the power of a matrix a to the nth power
    fn pow(mut a: Matrix, mut n: i32) -> Matrix {
        let mut res = [[0; SIZE]; SIZE];
        for i in 0..SIZE {
            res[i][i] = 1; // Identity matrix
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

    // Main function to compute the length after transformations
    fn length_after_transformations(s: &str, t: i32, nums: &[i32]) -> i32 {
        let mut m = [[0; SIZE]; SIZE];
        for i in 0..SIZE {
            for j in i + 1..=i + nums[i] as usize {
                m[i][j % SIZE] = 1;
            }
        }
        let m = Self::pow(m, t);

        let mut cnt = [0; SIZE];
        for c in s.chars() {
            cnt[(c as u8 - b'a') as usize] += 1;
        }

        let mut ans = 0i64;
        for i in 0..SIZE {
            let row_sum: i64 = m[i].iter().sum();
            ans = (ans + row_sum * cnt[i] as i64) % MOD;
        }
        ans as i32
    }
}

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    // Parse input
    let s_len: usize = lines.next().unwrap().parse().unwrap();
    let s = lines.next().unwrap();
    let t: i32 = lines.next().unwrap().parse().unwrap();
    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Ensure input matches expectations
    assert_eq!(s.len(), s_len);
    assert_eq!(nums.len(), SIZE);

    // Solve the problem
    let solution = Solution;
    let result = solution.length_after_transformations(s, t, &nums);

    // Output the result
    println!("{}", result);
}