use std::io::{self, BufRead, Write};

const MOD: i64 = 1_000_000_007;
const SIZE: usize = 26;

type Matrix = [[i64; SIZE]; SIZE];

struct Solution;

impl Solution {
    // Returns the result of multiplying matrix a and matrix b
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

    // Returns the result of raising matrix a to the power of n
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

    // Computes the length after transformations
    pub fn length_after_transformations(s: &str, t: i32, nums: &[i32; SIZE]) -> i64 {
        let mut m = [[0; SIZE]; SIZE];
        for i in 0..SIZE {
            for j in (i + 1)..=(i + nums[i] as usize) {
                m[i][j % SIZE] = 1;
            }
        }
        let m = Self::pow(m, t);

        let mut cnt = [0; SIZE];
        for c in s.chars() {
            cnt[(c as u8 - b'a') as usize] += 1;
        }

        let mut ans: i64 = 0;
        for i in 0..SIZE {
            // Sum of the i-th row of m is f[t][i]
            ans += m[i].iter().sum::<i64>() * cnt[i] as i64;
        }
        ans % MOD
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut stdout = io::stdout();
    let mut stdout_lock = stdout.lock();

    let mut input = String::new();
    stdin_lock.read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let s_len: usize = iter.next().unwrap().parse().unwrap();
    let s: String = iter.next().unwrap().to_string();
    let t: i32 = iter.next().unwrap().parse().unwrap();

    let mut nums = [0; SIZE];
    for i in 0..SIZE {
        input.clear();
        stdin_lock.read_line(&mut input).unwrap();
        nums[i] = input.trim().parse().unwrap();
    }

    let solution = Solution;
    let result = solution.length_after_transformations(&s, t, &nums);
    writeln!(stdout_lock, "{}", result).unwrap();
}