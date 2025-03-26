use std::io;
use std::io::prelude::*;

const MOD: i64 = 1_000_000_007;
const SIZE: usize = 26;

type Matrix = [[i64; SIZE]; SIZE];

fn mul(a: &Matrix, b: &Matrix) -> Matrix {
    let mut c: Matrix = [[0; SIZE]; SIZE];
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

fn pow(mut a: Matrix, mut n: i32) -> Matrix {
    let mut res: Matrix = [[0; SIZE]; SIZE];
    for i in 0..SIZE {
        res[i][i] = 1; // 单位矩阵
    }
    while n > 0 {
        if (n & 1) != 0 {
            res = mul(&res, &a);
        }
        a = mul(&a, &a);
        n >>= 1;
    }
    res
}

fn length_after_transformations(s: String, t: i32, nums: &Vec<i32>) -> i64 {
    let mut m: Matrix = [[0; SIZE]; SIZE];
    for i in 0..SIZE {
        for j in i + 1..=i + nums[i] as usize {
            m[i][j % SIZE] = 1;
        }
    }
    let m = pow(m, t);

    let mut cnt: [i32; SIZE] = [0; SIZE];
    for c in s.chars() {
        cnt[(c as u8 - b'a') as usize] += 1;
    }

    let mut ans: i64 = 0;
    for i in 0..SIZE {
        ans = (ans + m[i].iter().sum::<i64>() * cnt[i] as i64) % MOD;
    }
    ans
}

fn main() -> io::Result<()> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let mut iter = s.trim().split_whitespace();
    let _s_len: i32 = iter.next().unwrap().parse().unwrap();

    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let s = s.trim().to_string();

    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let t: i32 = s.trim().parse().unwrap();

    let mut nums: Vec<i32> = Vec::new();
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let nums_str: Vec<&str> = s.trim().split_whitespace().collect();

    for num_str in nums_str {
        nums.push(num_str.parse().unwrap());
    }

    let result = length_after_transformations(s, t, &nums);
    println!("{}", result);

    Ok(())
}