// LeetCode Weekly Contest 421 Problem 4 in Rust
//
// REQUIREMENTS:
// 1. Translates the ENTIRE file from C to Rust as a complete program
// 2. Preserves the algorithm logic exactly
// 3. Uses idiomatic Rust with proper error handling
// 4. Maintains the EXACT SAME stdin/stdout format
// 5. Adds helpful comments where needed

use std::io::{self, Read};

// Constants equivalent to the C defines
const MODULO_VAL: i64 = 1000000007;
const RADIX_VAL: usize = 26;

// Matrix structure equivalent to the C typedef
#[derive(Clone, Debug)]
struct Matrix {
    m: [[i32; RADIX_VAL]; RADIX_VAL],
}

impl Matrix {
    // Creates a new Matrix initialized to all zeros
    fn new() -> Self {
        Matrix {
            m: [[0; RADIX_VAL]; RADIX_VAL],
        }
    }
}

// Equivalent to the C matrixMultiply function
fn matrix_multiply(a: &Matrix, b: &Matrix, result: &mut Matrix) {
    for x in 0..RADIX_VAL {
        for y in 0..RADIX_VAL {
            let mut sum: i64 = 0;
            for z in 0..RADIX_VAL {
                sum = (sum + (a.m[x][z] as i64) * (b.m[z][y] as i64)) % MODULO_VAL;
            }
            result.m[x][y] = sum as i32;
        }
    }
}

// Equivalent to the C lengthAfterTransformations function
fn length_after_transformations(s: &str, t: i32, nums: &[i32], _nums_size: usize) -> i32 {
    let mut src = [0i32; RADIX_VAL];
    // Count the frequency of each character in s
    for c in s.chars() {
        let idx = (c as u8 - b'a') as usize;
        src[idx] += 1;
    }

    // Initialize matrices
    let mut init = Matrix::new();
    let mut dp = [Matrix::new(), Matrix::new()];

    // Prepare dp[0] as the identity matrix, and also build init
    for x in 0..RADIX_VAL {
        dp[0].m[x][x] = 1;
        // For y from 1 to nums[x], add transitions in init
        for y in 1..=(nums[x]) {
            let mut z = x as i32 + y;
            if z >= RADIX_VAL as i32 {
                z -= RADIX_VAL as i32;
            }
            init.m[z as usize][x] = 1;
        }
    }

    // Extract bits of t into digits
    let mut digits = Vec::new();
    let mut temp_t = t;
    while temp_t != 0 {
        digits.push(temp_t & 1);
        temp_t >>= 1;
    }

    // Perform repeated squaring using the bits of t
    let mut z_index = 0;
    for &bit in digits.iter().rev() {
        // dp[1 - z_index] = dp[z_index] * dp[z_index]
        let mut tmp = Matrix::new();
        matrix_multiply(&dp[z_index], &dp[z_index], &mut tmp);
        dp[1 - z_index] = tmp;

        if bit == 1 {
            // dp[z_index] = dp[1 - z_index] * init
            let mut tmp2 = Matrix::new();
            matrix_multiply(&dp[1 - z_index], &init, &mut tmp2);
            dp[z_index] = tmp2;
        } else {
            // If bit is 0, just swap z_index
            z_index = 1 - z_index;
        }
    }

    // Compute the final result based on dp[z_index] and the character counts
    let dp_final = &dp[z_index];
    let mut result: i64 = 0;
    for x in 0..RADIX_VAL {
        for y in 0..RADIX_VAL {
            let contribution = (dp_final.m[x][y] as i64) * (src[y] as i64);
            result = (result + contribution) % MODULO_VAL;
        }
    }

    result as i32
}

fn main() {
    // Read all input tokens
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let tokens: Vec<&str> = input.split_whitespace().collect();

    // Parse inputs according to the C code's logic
    let mut idx = 0;
    let s_len: usize = tokens[idx].parse().unwrap();
    idx += 1;

    // Read the string s (assuming it has length at most s_len)
    let s = tokens[idx];
    idx += 1;

    // Read t
    let t: i32 = tokens[idx].parse().unwrap();
    idx += 1;

    // nums_size is fixed at 26 in the C code
    let nums_size = 26;
    let mut nums = vec![0i32; nums_size];
    for i in 0..nums_size {
        nums[i] = tokens[idx].parse().unwrap();
        idx += 1;
    }

    // Compute the result
    let result = length_after_transformations(s, t, &nums, nums_size);

    // Print the result
    println!("{}", result);
}