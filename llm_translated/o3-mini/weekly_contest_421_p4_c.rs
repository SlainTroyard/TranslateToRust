use std::error::Error;
use std::io::{self, Read};

// The constant modulo and radix value per the C code.
const MODULO_VAL: i64 = 1_000_000_007;
const RADIX_VAL: usize = 26;

// Define Matrix struct containing a 26x26 matrix.
#[derive(Clone)]
struct Matrix {
    m: [[i64; RADIX_VAL]; RADIX_VAL],
}

impl Matrix {
    // Create a new Matrix with all entries initialized to zero.
    fn new() -> Self {
        Matrix {
            m: [[0; RADIX_VAL]; RADIX_VAL],
        }
    }
}

/// Multiply two matrices modulo MODULO_VAL.
///
/// The result is computed as:
///   result[x][y] = sum(a[x][z] * b[z][y]) % MODULO_VAL
fn matrix_multiply(a: &Matrix, b: &Matrix) -> Matrix {
    let mut result = Matrix::new();
    for x in 0..RADIX_VAL {
        for y in 0..RADIX_VAL {
            let mut sum = 0i64;
            for z in 0..RADIX_VAL {
                sum = (sum + a.m[x][z] * b.m[z][y]) % MODULO_VAL;
            }
            result.m[x][y] = sum;
        }
    }
    result
}

/// Compute the length after transformations according to the problem logic.
///
/// s: input string (only lowercase letters).
/// t: number of transformation steps.
/// nums: transformation numbers for each character.
fn length_after_transformations(s: &str, t: i32, nums: &[i32]) -> i32 {
    // src counts the frequency of each character in s.
    let mut src = [0i64; RADIX_VAL];
    // digits will hold up to 32 binary digits of t.
    let mut digits = [0; 32];
    let mut digits_size = 0;
    let mut result = 0i64;

    // init: transformation matrix based on nums;
    let mut init = Matrix::new();

    // dp[0] and dp[1] are used for exponentiation by squaring.
    let mut dp = [Matrix::new(), Matrix::new()];

    // Initialization: dp[0] as identity matrix.
    for x in 0..RADIX_VAL {
        dp[0].m[x][x] = 1;
        // Build the transformation matrix 'init'
        // For every possible offset from 1 to nums[x]
        for y in 1..=nums[x] {
            // Compute the correct index wrapping around.
            let z = if x + (y as usize) < RADIX_VAL {
                x + (y as usize)
            } else {
                x + (y as usize) - RADIX_VAL
            };
            init.m[z][x] = 1;
        }
    }

    // Count frequency of each letter in the string.
    for ch in s.chars() {
        // ch should be a lowercase letter, so subtract 'a'
        src[(ch as u8 - b'a') as usize] += 1;
    }

    // Convert t to its binary representation.
    let mut x_val = t;
    while x_val != 0 {
        // Get lowest bit.
        digits[digits_size] = (x_val & 1) as i32;
        digits_size += 1;
        x_val >>= 1;
    }

    // Exponentiation using binary decomposition.
    let mut cur = 0; // Active index indicator for dp array.
    for d in (0..digits_size).rev() {
        // Square the matrix: dp[cur] = dp[cur] * dp[cur] and store it in the other slot.
        dp[1 - cur] = matrix_multiply(&dp[cur], &dp[cur]);

        // If the current binary digit is set, multiply by the transformation matrix 'init'
        if digits[d] == 1 {
            dp[cur] = matrix_multiply(&dp[1 - cur], &init);
        } else {
            // Otherwise, just switch to the squared matrix.
            cur = 1 - cur;
        }
    }

    // Compute the final answer by multiplying the resulting matrix with the src counts.
    // Sum over all rows and columns.
    for x in 0..RADIX_VAL {
        for y in 0..RADIX_VAL {
            result = (result + dp[cur].m[x][y] * src[y]) % MODULO_VAL;
        }
    }

    result as i32
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read the entire input from stdin.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Create an iterator over tokens splitting by whitespace.
    let mut tokens = input.split_whitespace();

    // Parse s_len (though it's not actually used to construct the string).
    let s_len: usize = tokens
        .next()
        .ok_or("Missing s_len")?
        .parse()
        .map_err(|_| "Failed to parse s_len")?;
    
    // Parse the string s.
    // The C code expects a string of length s_len; we assume the input is well-formed.
    let s = tokens
        .next()
        .ok_or("Missing string s")?
        .to_string();

    // Parse t.
    let t: i32 = tokens
        .next()
        .ok_or("Missing t")?
        .parse()
        .map_err(|_| "Failed to parse t")?;

    // There should be exactly 26 numbers in nums.
    let mut nums = Vec::with_capacity(RADIX_VAL);
    for _ in 0..RADIX_VAL {
        let num: i32 = tokens
            .next()
            .ok_or("Missing a number for nums")?
            .parse()
            .map_err(|_| "Failed to parse a number in nums")?;
        nums.push(num);
    }

    // Compute the result using the transformation function.
    let answer = length_after_transformations(&s, t, &nums);

    // Print the answer. The C code prints the result followed by a newline.
    println!("{}", answer);

    Ok(())
}