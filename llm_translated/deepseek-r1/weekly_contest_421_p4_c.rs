use std::io::{self, BufRead};

const MODULO_VAL: i64 = 1_000_000_007;

#[derive(Clone, Copy)]
struct Matrix {
    m: [[i32; 26]; 26],
}

impl Matrix {
    fn identity() -> Self {
        let mut m = [[0; 26]; 26];
        for i in 0..26 {
            m[i][i] = 1;
        }
        Matrix { m }
    }

    fn default() -> Self {
        Matrix { m: [[0; 26]; 26] }
    }
}

fn matrix_multiply(a: &Matrix, b: &Matrix, result: &mut Matrix) {
    for x in 0..26 {
        for y in 0..26 {
            let mut sum = 0i64;
            for z in 0..26 {
                sum += a.m[x][z] as i64 * b.m[z][y] as i64;
                sum %= MODULO_VAL;
            }
            result.m[x][y] = sum as i32;
        }
    }
}

fn length_after_transformations(s: &str, t: i32, nums: &[i32]) -> i32 {
    let mut src = [0; 26];
    for c in s.chars() {
        let idx = c as usize - b'a' as usize;
        src[idx] += 1;
    }

    let mut init = Matrix::default();
    for x in 0..26 {
        let steps = nums[x];
        for y in 1..=steps {
            let z = (x as i32 + y) % 26;
            let z = if z < 0 { z + 26 } else { z } as usize;
            init.m[z][x] = 1;
        }
    }

    let mut digits = Vec::new();
    let mut x = t;
    while x != 0 {
        digits.push(x & 1);
        x >>= 1;
    }
    let digits_size = digits.len();

    let mut dp = [Matrix::identity(), Matrix::default()];
    let mut z = 0;

    for i in (0..digits_size).rev() {
        matrix_multiply(&dp[z], &dp[z], &mut dp[1 - z]);

        if digits[i] == 1 {
            matrix_multiply(&dp[1 - z], &init, &mut dp[z]);
        } else {
            z = 1 - z;
        }
    }

    let mut total = 0i64;
    for x in 0..26 {
        for y in 0..26 {
            total = (total + dp[z].m[x][y] as i64 * src[y] as i64) % MODULO_VAL;
        }
    }

    total as i32
}

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    // Read s_len (unused)
    let mut s_len_str = String::new();
    stdin.read_line(&mut s_len_str).unwrap();
    let _s_len: usize = s_len_str.trim().parse().unwrap();

    // Read string s
    let mut s = String::new();
    stdin.read_line(&mut s).unwrap();
    let s = s.trim().to_string();

    // Read t
    let mut t_str = String::new();
    stdin.read_line(&mut t_str).unwrap();
    let t: i32 = t_str.trim().parse().unwrap();

    // Read nums (26 integers)
    let mut nums_line = String::new();
    stdin.read_line(&mut nums_line).unwrap();
    let nums: Vec<i32> = nums_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    assert_eq!(nums.len(), 26, "nums must have exactly 26 elements");

    let result = length_after_transformations(&s, t, &nums);
    println!("{}", result);
}