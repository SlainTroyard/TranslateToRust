use std::io;
use std::io::prelude::*;

const MOD: i64 = 1_000_000_007;
const SIZE: usize = 26;

type Matrix = [[i64; SIZE]; SIZE];

// 返回矩阵 a 和矩阵 b 相乘的结果
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

// 返回 n 个矩阵 a 相乘的结果
fn pow(mut a: Matrix, mut n: i32) -> Matrix {
    let mut res = [[0; SIZE]; SIZE];
    for i in 0..SIZE {
        res[i][i] = 1; // 单位矩阵
    }
    while n > 0 {
        if n & 1 == 1 {
            res = mul(&res, &a);
        }
        a = mul(&a, &a);
        n >>= 1;
    }
    res
}

fn length_after_transformations(s: &str, t: i32, nums: &Vec<i32>) -> i64 {
    let mut m = [[0; SIZE]; SIZE];
    for i in 0..SIZE {
        for j in i + 1..=i + nums[i] as usize {
            m[i][j % SIZE] = 1;
        }
    }
    m = pow(m, t);

    let mut cnt = [0; SIZE];
    for c in s.chars() {
        cnt[(c as u8 - b'a') as usize] += 1;
    }

    let mut ans: i64 = 0;
    for i in 0..SIZE {
        // m 第 i 行的和就是 f[t][i]
        ans += m[i].iter().sum::<i64>() * cnt[i];
    }
    ans % MOD
}

fn main() -> io::Result<()> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;

    let mut iter = s.trim().split_whitespace();
    let _s_len: i32 = iter.next().unwrap().parse().unwrap();
    let s_str: String = iter.next().unwrap().parse().unwrap();
    let t: i32 = iter.next().unwrap().parse().unwrap();

    let mut nums = Vec::new();
    let mut s = String::new();

    io::stdin().read_line(&mut s)?;

    for num_str in s.trim().split_whitespace() {
        nums.push(num_str.parse::<i32>().unwrap());
    }

    let result = length_after_transformations(&s_str, t, &nums);
    println!("{}", result);

    Ok(())
}