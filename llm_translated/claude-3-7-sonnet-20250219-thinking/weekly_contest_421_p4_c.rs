use std::io::{self, Read};

const MODULO_VAL: i32 = 1000000007;
const RADIX_VAL: usize = 26;

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

// Matrix multiplication function similar to the C version
fn matrix_multiply(a: &Matrix, b: &Matrix, result: &mut Matrix) {
    for x in 0..RADIX_VAL {
        for y in 0..RADIX_VAL {
            result.m[x][y] = 0;
            for z in 0..RADIX_VAL {
                result.m[x][y] = ((i64::from(a.m[x][z]) * i64::from(b.m[z][y]) + i64::from(result.m[x][y])) % i64::from(MODULO_VAL)) as i32;
            }
        }
    }
}

fn length_after_transformations(s: &str, t: i32, nums: &[i32], _nums_size: i32) -> i32 {
    let mut x: usize;
    let mut y: usize;
    let mut z: usize = 0;
    let mut digits_size: usize = 0;
    let mut result: i32 = 0;
    
    let mut src = [0; RADIX_VAL];
    let mut digits = [0; 32];
    let mut init = Matrix::new();
    let mut dp = [Matrix::new(), Matrix::new()];
    
    // Initialize dp[0] as identity matrix and init matrix based on nums
    for x in 0..RADIX_VAL {
        dp[0].m[x][x] = 1;
        for y in 1..=nums[x] as usize {
            z = if x + y >= RADIX_VAL { x + y - RADIX_VAL } else { x + y };
            init.m[z][x] = 1;
        }
    }
    
    // Count occurrences of each letter in the input string
    for ch in s.chars() {
        src[(ch as u8 - b'a') as usize] += 1;
    }
    
    // Extract binary digits of t
    let mut t_copy = t;
    while t_copy != 0 {
        digits[digits_size] = t_copy & 1;
        digits_size += 1;
        t_copy >>= 1;
    }
    
    // Use binary exponentiation to compute the result matrix
    z = 0;
    for x in (0..digits_size).rev() {
        matrix_multiply(&dp[z], &dp[z], &mut dp[1 - z]);
        if digits[x] == 1 {
            matrix_multiply(&dp[1 - z], &init, &mut dp[z]);
        } else {
            z = 1 - z;
        }
    }
    
    // Compute the final result
    for x in 0..RADIX_VAL {
        for y in 0..RADIX_VAL {
            result = ((i64::from(dp[z].m[x][y]) * i64::from(src[y])) + i64::from(result)) % i64::from(MODULO_VAL) as i32;
        }
    }
    
    result
}

fn main() {
    // Reading s_len
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read s_len");
    let s_len: usize = input.trim().parse().expect("Invalid s_len");
    
    // Reading s
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read s");
    let s = input.trim().to_string();
    
    // Reading t
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read t");
    let t: i32 = input.trim().parse().expect("Invalid t");
    
    // Reading nums (26 integers)
    let nums_size = 26;
    let mut nums = vec![0; nums_size];
    
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read nums");
    let nums_input: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid number in nums"))
        .collect();
    
    for i in 0..nums_size {
        nums[i] = nums_input[i];
    }
    
    // Computing and printing the result
    let result = length_after_transformations(&s, t, &nums, nums_size as i32);
    println!("{}", result);
}