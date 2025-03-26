use std::io::{self, BufRead};

const MODULO_VAL: i32 = 1_000_000_007;
const RADIX_VAL: usize = 26;

// Matrix struct for transformation calculations
struct Matrix {
    m: [[i32; RADIX_VAL]; RADIX_VAL],
}

impl Matrix {
    // Create a new zero-initialized matrix
    fn new() -> Self {
        Matrix {
            m: [[0; RADIX_VAL]; RADIX_VAL],
        }
    }
}

// Multiply two matrices and store the result in a third matrix
fn matrix_multiply(a: &Matrix, b: &Matrix) -> Matrix {
    let mut result = Matrix::new();
    
    for x in 0..RADIX_VAL {
        for y in 0..RADIX_VAL {
            result.m[x][y] = 0;
            for z in 0..RADIX_VAL {
                result.m[x][y] = ((a.m[x][z] as i64 * b.m[z][y] as i64 + result.m[x][y] as i64) % MODULO_VAL as i64) as i32;
            }
        }
    }
    
    result
}

fn length_after_transformations(s: &str, t: i32, nums: &[i32]) -> i32 {
    let mut src = [0; RADIX_VAL];
    let mut init = Matrix::new();
    let mut dp = [Matrix::new(), Matrix::new()];
    
    // Initialize identity matrix in dp[0]
    for x in 0..RADIX_VAL {
        dp[0].m[x][x] = 1;
    }
    
    // Initialize transformation matrix
    for x in 0..RADIX_VAL {
        for y in 1..=nums[x] as usize {
            let z = if x + y < RADIX_VAL { x + y } else { x + y - RADIX_VAL };
            init.m[z][x] = 1;
        }
    }
    
    // Count character frequencies in the input string
    for c in s.chars() {
        src[(c as u8 - b'a') as usize] += 1;
    }
    
    // Compute binary representation of t
    let mut digits = Vec::new();
    let mut t_copy = t;
    while t_copy != 0 {
        digits.push(t_copy & 1);
        t_copy >>= 1;
    }
    
    // Binary exponentiation to compute the transformation matrix after t iterations
    let mut z = 0;
    for &digit in digits.iter().rev() {
        dp[1 - z] = matrix_multiply(&dp[z], &dp[z]);
        if digit == 1 {
            dp[z] = matrix_multiply(&dp[1 - z], &init);
        } else {
            z = 1 - z;
        }
    }
    
    // Calculate the final result
    let mut result = 0;
    for x in 0..RADIX_VAL {
        for y in 0..RADIX_VAL {
            result = ((dp[z].m[x][y] as i64 * src[y] as i64 + result as i64) % MODULO_VAL as i64) as i32;
        }
    }
    
    result
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the string length (note: this isn't used directly in the original code)
    let s_len: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read the string
    let s = lines.next().unwrap().unwrap();
    
    // Read t
    let t: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read nums (always 26 elements)
    let nums_size = 26;
    let mut nums = Vec::with_capacity(nums_size);
    let nums_line = lines.next().unwrap().unwrap();
    for num_str in nums_line.split_whitespace() {
        nums.push(num_str.parse::<i32>().unwrap());
    }
    
    // Calculate and print the result
    let result = length_after_transformations(&s, t, &nums);
    println!("{}", result);
}