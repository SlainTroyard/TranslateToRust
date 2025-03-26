use std::io::{self, BufRead};
use std::cmp::min;

const MODULO_VAL: i32 = 1_000_000_007;
const RADIX_VAL: usize = 26;

#[derive(Clone, Debug)]
struct Matrix {
    m: [[i32; RADIX_VAL]; RADIX_VAL],
}

impl Matrix {
    fn new() -> Self {
        Matrix {
            m: [[0; RADIX_VAL]; RADIX_VAL],
        }
    }
}

fn matrix_multiply(a: &Matrix, b: &Matrix) -> Matrix {
    let mut result = Matrix::new();
    for x in 0..RADIX_VAL {
        for y in 0..RADIX_VAL {
            result.m[x][y] = 0;
            for z in 0..RADIX_VAL {
                result.m[x][y] = (result.m[x][y] + (a.m[x][z] as i64 * b.m[z][y] as i64 % MODULO_VAL as i64) as i32) % MODULO_VAL;
            }
        }
    }
    result
}

fn length_after_transformations(s: &str, t: i32, nums: &[i32]) -> i32 {
    let mut src = [0; RADIX_VAL];
    let mut init = Matrix::new();
    let mut dp = [Matrix::new(), Matrix::new()];

    for x in 0..RADIX_VAL {
        dp[0].m[x][x] = 1;
        for y in 1..=nums[x] {
            let z = if x + y < RADIX_VAL { x + y } else { x + y - RADIX_VAL };
            init.m[z][x] = 1;
        }
    }

    for ch in s.chars() {
        src[ch as usize - 'a' as usize] += 1;
    }

    let mut digits = Vec::new();
    let mut x = t;
    while x != 0 {
        digits.push(x & 1);
        x >>= 1;
    }

    let mut z = 0;
    for &digit in digits.iter().rev() {
        dp[1 - z] = matrix_multiply(&dp[z], &dp[z]);
        if digit == 1 {
            dp[z] = matrix_multiply(&dp[1 - z], &init);
        } else {
            z = 1 - z;
        }
    }

    let mut result = 0;
    for x in 0..RADIX_VAL {
        for y in 0..RADIX_VAL {
            result = (result + (dp[z].m[x][y] as i64 * src[y] as i64 % MODULO_VAL as i64) as i32) % MODULO_VAL;
        }
    }

    result
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the length of the string (not used directly)
    let s_len: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read the string `s`
    let s = lines.next().unwrap().unwrap();

    // Read the number of transformations `t`
    let t: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read the `nums` array
    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Ensure nums has the correct size
    assert_eq!(nums.len(), RADIX_VAL);

    // Compute the result
    let result = length_after_transformations(&s, t, &nums);

    // Print the result
    println!("{}", result);
}