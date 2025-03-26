use std::io::{self, BufRead};

const MOD: i32 = 1_000_000_007;
const SIZE: usize = 26;

type Matrix = [[i32; SIZE]; SIZE];

struct Solution {}

impl Solution {
    // Return the result of multiplying matrix a and matrix b
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

    // Return the result of multiplying n matrices a
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
            // The sum of row i in m is f[t][i]
            let row_sum: i64 = m[i].iter().map(|&x| x as i64).sum();
            ans += row_sum * (cnt[i] as i64);
        }
        (ans % (MOD as i64)) as i32
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Parse first line: s_len s t
    let line = lines.next().unwrap()?;
    let parts: Vec<&str> = line.split_whitespace().collect();
    let _s_len: usize = parts[0].parse().unwrap();
    let s: String = parts[1].to_string();
    let t: i32 = parts[2].parse().unwrap();
    
    // Parse second line: nums
    let line = lines.next().unwrap()?;
    let nums: Vec<i32> = line.split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    
    let solution = Solution{};
    println!("{}", solution.length_after_transformations(s, t, nums));
    
    Ok(())
}