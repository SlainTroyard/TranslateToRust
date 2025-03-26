use std::io::{self, BufRead};

const MOD: i64 = 1_000_000_007;
const SIZE: usize = 26;

type Matrix = [[i64; SIZE]; SIZE];

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

fn pow(mut a: Matrix, mut n: i32) -> Matrix {
    let mut res = [[0; SIZE]; SIZE];
    for i in 0..SIZE {
        res[i][i] = 1; // Identity matrix
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

fn length_after_transformations(s: &str, t: i32, nums: &[i32]) -> i64 {
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
        ans += m[i].iter().sum::<i64>() * cnt[i] as i64;
    }
    ans % MOD
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let s_len: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let s = lines.next().unwrap().unwrap();
    let t: i32 = lines.next().unwrap().unwrap().parse().unwrap();

    let mut nums = [0; SIZE];
    for i in 0..SIZE {
        nums[i] = lines.next().unwrap().unwrap().parse().unwrap();
    }

    let result = length_after_transformations(&s, t, &nums);
    println!("{}", result);
}