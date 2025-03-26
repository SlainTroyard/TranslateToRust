use std::io::{self, BufRead};

const MOD: i32 = 1_000_000_007;
const SIZE: usize = 26;

type Matrix = [[i32; SIZE]; SIZE];

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
                    c[i][j] = (c[i][j] + ((a[i][k] as i64) * (b[k][j] as i64)) as i32) % MOD;
                }
            }
        }
        c
    }

    // Returns the result of multiplying matrix a by itself n times
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

    pub fn length_after_transformations(s: String, t: i32, nums: Vec<i32>) -> i32 {
        let mut m = [[0; SIZE]; SIZE];
        for i in 0..SIZE {
            for j in i + 1..=i + nums[i] as usize {
                m[i][j % SIZE] = 1;
            }
        }
        let m = Self::pow(m, t);

        let mut cnt = [0; SIZE];
        for c in s.chars() {
            cnt[(c as usize) - ('a' as usize)] += 1;
        }

        let mut ans: i64 = 0;
        for i in 0..SIZE {
            // Sum of row i in matrix m is f[t][i]
            ans += m[i].iter().map(|&x| x as i64).sum::<i64>() * (cnt[i] as i64);
        }
        (ans % (MOD as i64)) as i32
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read first line containing s_len and s
    let line = lines.next().unwrap()?;
    let parts: Vec<&str> = line.split_whitespace().collect();
    let _s_len: usize = parts[0].parse().unwrap();
    let s = parts[1].to_string();
    
    // Read t
    let t: i32 = lines.next().unwrap()?.trim().parse().unwrap();
    
    // Read nums
    let line = lines.next().unwrap()?;
    let nums: Vec<i32> = line.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    
    let solution = Solution;
    println!("{}", solution.length_after_transformations(s, t, nums));
    
    Ok(())
}