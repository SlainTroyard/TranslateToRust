// Problem: Weekly Contest 421 Problem 4

const MODULO_VAL: i64 = 1_000_000_007;
const RADIX_VAL: usize = 26;

#[derive(Clone)]
struct Matrix {
    m: [[i64; RADIX_VAL]; RADIX_VAL],
}

impl Matrix {
    fn new() -> Self {
        Matrix { m: [[0; RADIX_VAL]; RADIX_VAL] }
    }
}

fn matrix_multiply(a: &Matrix, b: &Matrix) -> Matrix {
    let mut result = Matrix::new();
    for x in 0..RADIX_VAL {
        for y in 0..RADIX_VAL {
            for z in 0..RADIX_VAL {
                result.m[x][y] = (result.m[x][y] + a.m[x][z] * b.m[z][y]) % MODULO_VAL;
            }
        }
    }
    result
}

fn length_after_transformations(s: &str, t: i32, nums: &[i32]) -> i64 {
    let mut src = [0; RADIX_VAL];
    let mut digits = [0; 32];
    let mut digits_size = 0;
    let mut init = Matrix::new();
    let mut dp = [Matrix::new(), Matrix::new()];

    // Initialize dp[0] and init
    for x in 0..RADIX_VAL {
        dp[0].m[x][x] = 1;
        for y in 1..=nums[x] {
            let z = if x + y as usize < RADIX_VAL { x + y as usize } else { x + y as usize - RADIX_VAL };
            init.m[z][x] = 1;
        }
    }

    // Count character frequencies
    for c in s.chars() {
        src[c as usize - 'a' as usize] += 1;
    }

    // Convert t to binary
    let mut t_copy = t;
    while t_copy != 0 {
        digits[digits_size] = t_copy & 1;
        digits_size += 1;
        t_copy >>= 1;
    }

    // Calculate result
    let mut z = 0;
    for x in (0..digits_size).rev() {
        dp[1 - z] = matrix_multiply(&dp[z], &dp[z]);
        if digits[x] == 1 {
            dp[z] = matrix_multiply(&dp[1 - z], &init);
        } else {
            z = 1 - z;
        }
    }

    // Calculate final result
    let mut result = 0;
    for x in 0..RADIX_VAL {
        for y in 0..RADIX_VAL {
            result = (result + dp[z].m[x][y] * src[y] as i64) % MODULO_VAL;
        }
    }
    result
}

fn main() -> std::io::Result<()> {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read s_len
    let s_len: usize = lines.next().unwrap()?.parse().unwrap();

    // Read s
    let s = lines.next().unwrap()?;

    // Read t
    let t: i32 = lines.next().unwrap()?.parse().unwrap();

    // Read nums
    let nums: Vec<i32> = lines.next().unwrap()?
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Calculate and print result
    let result = length_after_transformations(&s, t, &nums);
    println!("{}", result);

    Ok(())
}