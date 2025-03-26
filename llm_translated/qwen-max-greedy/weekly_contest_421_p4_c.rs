use std::io::{self, BufRead};

const MODULO_VAL: i64 = 1_000_000_007;
const RADIX_VAL: usize = 26;

#[derive(Clone, Copy)]
struct Matrix {
    m: [[i64; RADIX_VAL]; RADIX_VAL],
}

impl Matrix {
    fn new() -> Self {
        Matrix { m: [[0; RADIX_VAL]; RADIX_VAL] }
    }

    fn identity() -> Self {
        let mut matrix = Matrix::new();
        for i in 0..RADIX_VAL {
            matrix.m[i][i] = 1;
        }
        matrix
    }

    fn multiply(&self, other: &Matrix) -> Matrix {
        let mut result = Matrix::new();
        for i in 0..RADIX_VAL {
            for j in 0..RADIX_VAL {
                for k in 0..RADIX_VAL {
                    result.m[i][j] = (result.m[i][j] + self.m[i][k] * other.m[k][j]) % MODULO_VAL;
                }
            }
        }
        result
    }
}

fn length_after_transformations(s: &str, t: i32, nums: &[i32]) -> i64 {
    let mut src = [0; RADIX_VAL];
    for c in s.chars() {
        if let Some(index) = c as usize - 'a' as usize {
            src[index] += 1;
        }
    }

    let mut init = Matrix::new();
    for x in 0..RADIX_VAL {
        for y in 1..=nums[x] as usize {
            let z = if x + y < RADIX_VAL { x + y } else { x + y - RADIX_VAL };
            init.m[z][x] = 1;
        }
    }

    let mut dp = [Matrix::identity(), Matrix::new()];
    let mut digits = Vec::new();
    let mut t = t;
    while t > 0 {
        digits.push(t & 1);
        t >>= 1;
    }

    let mut z = 0;
    for &digit in digits.iter().rev() {
        dp[1 - z] = dp[z].multiply(&dp[z]);
        if digit == 1 {
            dp[z] = dp[1 - z].multiply(&init);
        } else {
            z = 1 - z;
        }
    }

    let mut result = 0;
    for x in 0..RADIX_VAL {
        for y in 0..RADIX_VAL {
            result = (result + dp[z].m[x][y] * src[y] as i64) % MODULO_VAL;
        }
    }
    result
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let s_len: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let s: String = lines.next().unwrap().unwrap();
    let t: i32 = lines.next().unwrap().unwrap().parse().unwrap();
    let nums: Vec<i32> = (0..RADIX_VAL)
        .map(|_| lines.next().unwrap().unwrap().parse().unwrap())
        .collect();

    let result = length_after_transformations(&s, t, &nums);
    println!("{}", result);
}