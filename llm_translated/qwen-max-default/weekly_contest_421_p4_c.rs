use std::io::{self, BufRead};

const MODULO_VAL: usize = 1000000007;
const RADIX_VAL: usize = 26;

#[derive(Clone, Copy)]
struct Matrix {
    m: [[usize; RADIX_VAL]; RADIX_VAL],
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
}

fn matrix_multiply(a: &Matrix, b: &Matrix) -> Matrix {
    let mut result = Matrix::new();
    for x in 0..RADIX_VAL {
        for y in 0..RADIX_VAL {
            for z in 0..RADIX_VAL {
                result.m[x][y] = (result.m[x][y] + (a.m[x][z] as u64 * b.m[z][y] as u64) % MODULO_VAL as u64) as usize % MODULO_VAL;
            }
        }
    }
    result
}

fn length_after_transformations(s: &str, t: usize, nums: &[usize]) -> usize {
    let mut src = [0; RADIX_VAL];
    let mut init = Matrix::new();
    let mut dp = [Matrix::identity(), Matrix::new()];

    for x in 0..RADIX_VAL {
        for y in 1..=nums[x] {
            let z = if x + y < RADIX_VAL { x + y } else { x + y - RADIX_VAL };
            init.m[z][x] = 1;
        }
    }

    for c in s.chars() {
        src[(c as u8 - b'a') as usize] += 1;
    }

    let mut digits = Vec::new();
    let mut t = t;
    while t > 0 {
        digits.push(t & 1);
        t >>= 1;
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
            result = (result + (dp[z].m[x][y] as u64 * src[y] as u64) % MODULO_VAL as u64) as usize % MODULO_VAL;
        }
    }
    result
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let s_len: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let s: String = lines.next().unwrap().unwrap();
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let nums: Vec<usize> = lines.next().unwrap().unwrap()
                                .split_whitespace()
                                .map(|x| x.parse().unwrap())
                                .collect();

    let result = length_after_transformations(&s, t, &nums);
    println!("{}", result);
}