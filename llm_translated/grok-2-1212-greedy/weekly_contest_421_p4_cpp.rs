use std::io::{self, BufRead};

const MOD: i64 = 1_000_000_007;
const SIZE: usize = 26;

type Matrix = [[i64; SIZE]; SIZE];

struct Solution;

impl Solution {
    // Multiply two matrices
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

    // Calculate the power of a matrix
    fn pow(mut a: Matrix, mut n: i64) -> Matrix {
        let mut res = [[0; SIZE]; SIZE];
        for i in 0..SIZE {
            res[i][i] = 1; // Identity matrix
        }
        while n > 0 {
            if n & 1 == 1 {
                res = Solution::mul(&res, &a);
            }
            a = Solution::mul(&a, &a);
            n >>= 1;
        }
        res
    }

    // Calculate the length after transformations
    fn length_after_transformations(s: &str, t: i64, nums: &[i32]) -> i64 {
        let mut m = [[0; SIZE]; SIZE];
        for i in 0..SIZE {
            for j in i + 1..=i + nums[i] as usize {
                m[i][j % SIZE] = 1;
            }
        }
        let m = Solution::pow(m, t);

        let mut cnt = [0; SIZE];
        for c in s.chars() {
            cnt[c as usize - 'a' as usize] += 1;
        }

        let mut ans = 0;
        for i in 0..SIZE {
            // Sum of the i-th row of m is f[t][i]
            ans += m[i].iter().sum::<i64>() * cnt[i];
        }
        ans % MOD
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read input
    let s_len: usize = lines.next().unwrap()?.parse().unwrap();
    let s: String = lines.next().unwrap()?;
    let t: i64 = lines.next().unwrap()?.parse().unwrap();
    let nums: Vec<i32> = (0..26)
        .map(|_| lines.next().unwrap()?.parse().unwrap())
        .collect();

    // Calculate and print result
    let solution = Solution;
    let result = solution.length_after_transformations(&s, t, &nums);
    println!("{}", result);

    Ok(())
}