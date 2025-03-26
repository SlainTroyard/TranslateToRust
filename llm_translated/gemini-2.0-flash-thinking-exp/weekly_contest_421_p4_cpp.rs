// Problem: Weekly Contest 421 Problem 4
use std::io::{self, BufRead};

const MOD: i32 = 1_000_000_007;
const SIZE: usize = 26;

type Matrix = [[i32; SIZE]; SIZE];

// 返回矩阵 a 和矩阵 b 相乘的结果
fn mul(a: &Matrix, b: &Matrix) -> Matrix {
    let mut c: Matrix = [[0; SIZE]; SIZE];
    for i in 0..SIZE {
        for k in 0..SIZE {
            if a[i][k] == 0 {
                continue;
            }
            for j in 0..SIZE {
                c[i][j] = (c[i][j] as i64 + (a[i][k] as i64) * (b[k][j] as i64)) as i32 % MOD;
            }
        }
    }
    c
}

// 返回 n 个矩阵 a 相乘的结果
fn pow(mut a: Matrix, mut n: i32) -> Matrix {
    let mut res: Matrix = [[0; SIZE]; SIZE];
    for i in 0..SIZE {
        res[i][i] = 1; // 单位矩阵
    }
    while n > 0 {
        if (n & 1) == 1 {
            res = mul(&res, &a);
        }
        a = mul(&a, &a);
        n >>= 1;
    }
    res
}

struct Solution {}

impl Solution {
    fn length_after_transformations(s: String, t: i32, nums: Vec<i32>) -> i32 {
        let mut m: Matrix = [[0; SIZE]; SIZE];
        for i in 0..SIZE {
            for j in (i + 1)..=(i + nums[i] as usize) {
                m[i][j % SIZE] = 1;
            }
        }
        let m_pow_t = pow(m, t);

        let mut cnt: [i32; SIZE] = [0; SIZE];
        for c in s.chars() {
            cnt[(c as u8 - b'a') as usize] += 1;
        }

        let mut ans: i64 = 0;
        for i in 0..SIZE {
            // m 第 i 行的和就是 f[t][i]
            ans += m_pow_t[i].iter().sum::<i32>() as i64 * cnt[i] as i64;
        }
        (ans % MOD as i64) as i32
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let s_len: usize = lines.next().unwrap().expect("Failed to read line").trim().parse().expect("Failed to parse s_len");
    let s: String = lines.next().unwrap().expect("Failed to read line").trim().to_string();
    let t: i32 = lines.next().unwrap().expect("Failed to read line").trim().parse().expect("Failed to parse t");

    let mut nums: Vec<i32> = Vec::new();
    let nums_line = lines.next().unwrap().expect("Failed to read line");
    for num_str in nums_line.trim().split_whitespace() {
        nums.push(num_str.parse().expect("Failed to parse num"));
    }

    let solution = Solution {};
    println!("{}", solution.length_after_transformations(s, t, nums));
}