use std::io::{self, Write};

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
}

fn matrix_multiply(a: &Matrix, b: &Matrix, result: &mut Matrix) {
    for x in 0..RADIX_VAL {
        for y in 0..RADIX_VAL {
            result.m[x][y] = 0;
            for z in 0..RADIX_VAL {
                result.m[x][y] = (a.m[x][z] * b.m[z][y] + result.m[x][y]) % MODULO_VAL;
            }
        }
    }
}

fn length_after_transformations(s: &str, t: i32, nums: &[i32]) -> i64 {
    let mut src = [0; RADIX_VAL];
    let mut digits = [0; 32];
    let mut init = Matrix::new();
    let mut dp = [Matrix::new(), Matrix::new()];
    
    for x in 0..RADIX_VAL {
        dp[0].m[x][x] = 1;
        for y in 1..=nums[x] as usize {
            let z = if x + y < RADIX_VAL { x + y } else { x + y - RADIX_VAL };
            init.m[z][x] = 1;
        }
    }
    
    for c in s.chars() {
        src[(c as u8 - b'a') as usize] += 1;
    }
    
    let mut digits_size = 0;
    let mut x = t;
    while x != 0 {
        digits[digits_size] = x & 1;
        digits_size += 1;
        x >>= 1;
    }
    
    let mut z = 0;
    for i in (0..digits_size).rev() {
        matrix_multiply(&dp[z], &dp[z], &mut dp[1 - z]);
        if digits[i] == 1 {
            matrix_multiply(&dp[1 - z], &init, &mut dp[z]);
        } else {
            z = 1 - z;
        }
    }
    
    let mut result = 0;
    for x in 0..RADIX_VAL {
        for y in 0..RADIX_VAL {
            result = (dp[z].m[x][y] * src[y] as i64 + result) % MODULO_VAL;
        }
    }
    
    result
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let s_len: usize = input.trim().parse().expect("Invalid input");
    
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let s = input.trim();
    
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let t: i32 = input.trim().parse().expect("Invalid input");
    
    let mut nums = [0; RADIX_VAL];
    for i in 0..RADIX_VAL {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        nums[i] = input.trim().parse().expect("Invalid input");
    }
    
    let result = length_after_transformations(s, t, &nums);
    println!("{}", result);
}