use std::io::{self, BufRead, Write};

const MOD: i64 = 1_000_000_007;
const SIZE: usize = 26;

type Matrix = [[i64; SIZE]; SIZE];

struct Solution;

impl Solution {
    // 返回矩阵 a 和矩阵 b 相乘的结果
    fn mul(a: &Matrix, b: &Matrix) -> Matrix {
        let mut c = [[0; SIZE]; SIZE];
        for i in 0..SIZE {
            for k in 0..SIZE {
                if a[i][k] == 0 {
                    continue;
                }
                for j in 0..SIZE {
                    c[i][j] = (c[i][j] + (a[i][k] * b[k][j]) % MOD) % MOD;
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
                res = Self::mul(&res, &a);
            }
            a = Self::mul(&a, &a);
            n >>= 1;
        }
        res
    }

    pub fn length_after_transformations(s: &str, t: i32, nums: &[i32]) -> i64 {
        let mut m = [[0; SIZE]; SIZE];
        for i in 0..SIZE {
            for j in i + 1..=i + nums[i] as usize {
                m[i][j % SIZE] = 1;
            }
        }
        let m = Self::pow(m, t);

        let mut cnt = [0; SIZE];
        for c in s.chars() {
            cnt[c as usize - 'a' as usize] += 1;
        }

        let mut ans: i64 = 0;
        for i in 0..SIZE {
            // m 第 i 行的和就是 f[t][i]
            ans += m[i].iter().sum::<i64>() * cnt[i] as i64;
        }
        ans % MOD
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer = String::new();

    stdin.lock().read_line(&mut buffer).unwrap();
    let mut iter = buffer.split_whitespace();
    let s_len: usize = iter.next().unwrap().parse().unwrap();
    let s: String = iter.next().unwrap().to_string();
    let t: i32 = iter.next().unwrap().parse().unwrap();

    let mut nums = vec![0; SIZE];
    for i in 0..SIZE {
        buffer.clear();
        stdin.lock().read_line(&mut buffer).unwrap();
        nums[i] = buffer.trim().parse().unwrap();
    }

    let solution = Solution;
    let result = solution.length_after_transformations(&s, t, &nums);
    writeln!(stdout, "{}", result).unwrap();
}