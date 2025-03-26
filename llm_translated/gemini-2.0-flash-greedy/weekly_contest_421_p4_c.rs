use std::io;
use std::io::Read;

const MODULO_VAL: i64 = 1_000_000_007;
const RADIX_VAL: usize = 26;

#[derive(Clone, Copy)]
struct Matrix {
    m: [[i64; RADIX_VAL]; RADIX_VAL],
}

impl Matrix {
    fn new() -> Self {
        Matrix {
            m: [[0; RADIX_VAL]; RADIX_VAL],
        }
    }
}

fn matrix_multiply(a: &Matrix, b: &Matrix, result: &mut Matrix) {
    for x in 0..RADIX_VAL {
        for y in 0..RADIX_VAL {
            result.m[x][y] = 0;
            for z in 0..RADIX_VAL {
                result.m[x][y] = ((a.m[x][z] * b.m[z][y]) % MODULO_VAL + result.m[x][y]) % MODULO_VAL;
            }
        }
    }
}

fn length_after_transformations(s: &str, t: i32, nums: &[i32]) -> i64 {
    let mut src = [0; RADIX_VAL];
    let mut init = Matrix::new();
    let mut dp = [Matrix::new(), Matrix::new()];

    for x in 0..RADIX_VAL {
        dp[0].m[x][x] = 1;
        for y in 1..=nums[x] {
            let z = if x + y as usize >= RADIX_VAL {
                x + y as usize - RADIX_VAL
            } else {
                x + y as usize
            };
            init.m[z][x] = 1;
        }
    }

    for &c in s.as_bytes() {
        src[(c - b'a') as usize] += 1;
    }

    let mut digits = [0; 32];
    let mut digits_size = 0;
    let mut x = t;
    while x != 0 {
        digits[digits_size] = x & 1;
        digits_size += 1;
        x >>= 1;
    }

    let mut z = 0;
    for x in (0..digits_size).rev() {
        let mut temp = Matrix::new();
        matrix_multiply(&dp[z], &dp[z], &mut temp);
        dp[1 - z] = temp;

        if digits[x] == 1 {
            let mut temp = Matrix::new();
            matrix_multiply(&dp[1 - z], &init, &mut temp);
            dp[z] = temp;
        } else {
            z = 1 - z;
        }
    }

    let mut result: i64 = 0;
    for x in 0..RADIX_VAL {
        for y in 0..RADIX_VAL {
            result = (dp[z].m[x][y] * src[y] as i64 + result) % MODULO_VAL;
        }
    }

    result
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut lines = input.lines();

    let s_len: usize = lines.next().unwrap().parse().unwrap();
    let s: String = lines.next().unwrap().to_string();
    let t: i32 = lines.next().unwrap().parse().unwrap();

    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let result = length_after_transformations(&s, t, &nums);
    println!("{}", result);

    Ok(())
}