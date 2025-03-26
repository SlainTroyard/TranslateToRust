use std::io::{self, BufRead};

const MODULO_VAL: i32 = 1_000_000_007;
const RADIX_VAL: usize = 26;

#[derive(Clone, Copy, Default)]
struct Matrix {
    m: [[i32; RADIX_VAL]; RADIX_VAL],
}

/// Multiplies two matrices and stores the result in the provided result matrix
fn matrix_multiply(a: &Matrix, b: &Matrix, result: &mut Matrix) {
    for x in 0..RADIX_VAL {
        for y in 0..RADIX_VAL {
            result.m[x][y] = 0;
            for z in 0..RADIX_VAL {
                result.m[x][y] = ((a.m[x][z] as i64 * b.m[z][y] as i64 + result.m[x][y] as i64) % MODULO_VAL as i64) as i32;
            }
        }
    }
}

fn length_after_transformations(s: &str, t: i32, nums: &[i32], _nums_size: i32) -> i32 {
    let mut src = [0; RADIX_VAL];
    let mut init = Matrix::default();
    let mut dp = [Matrix::default(), Matrix::default()];
    let mut digits = [0; 32];
    let mut digits_size = 0;
    
    // Initialize dp[0] as identity matrix
    for x in 0..RADIX_VAL {
        dp[0].m[x][x] = 1;
        
        // Initialize transformation matrix
        for y in 1..=nums[x] as usize {
            let z = if x + y < RADIX_VAL { x + y } else { x + y - RADIX_VAL };
            init.m[z][x] = 1;
        }
    }
    
    // Count character frequencies in the input string
    for ch in s.chars() {
        src[(ch as u8 - b'a') as usize] += 1;
    }
    
    // Convert t to binary representation
    let mut x = t;
    while x != 0 {
        digits[digits_size] = x & 1;
        digits_size += 1;
        x >>= 1;
    }
    
    // Binary exponentiation to compute the final transformation matrix
    let mut z = 0;
    for x in (0..digits_size).rev() {
        matrix_multiply(&dp[z], &dp[z], &mut dp[1 - z]);
        if digits[x] == 1 {
            matrix_multiply(&dp[1 - z], &init, &mut dp[z]);
        } else {
            z = 1 - z;
        }
    }
    
    // Compute the final result
    let mut result = 0;
    for x in 0..RADIX_VAL {
        for y in 0..RADIX_VAL {
            result = ((dp[z].m[x][y] as i64 * src[y] as i64 + result as i64) % MODULO_VAL as i64) as i32;
        }
    }
    
    result
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read string length
    let s_len: usize = lines.next().unwrap()?.trim().parse().unwrap();
    
    // Read the string
    let s = lines.next().unwrap()?;
    
    // Read t
    let t: i32 = lines.next().unwrap()?.trim().parse().unwrap();
    
    // Read nums array (always 26 elements)
    let nums_size = 26;
    let mut nums = vec![0; nums_size];
    let nums_input = lines.next().unwrap()?;
    for (i, num_str) in nums_input.split_whitespace().enumerate() {
        if i < nums_size {
            nums[i] = num_str.parse().unwrap();
        }
    }
    
    // Calculate and print the result
    let result = length_after_transformations(&s, t, &nums, nums_size as i32);
    println!("{}", result);
    
    Ok(())
}