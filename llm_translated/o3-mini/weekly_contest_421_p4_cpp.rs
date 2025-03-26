use std::error::Error;
use std::io::{self, BufRead};

const MOD: i64 = 1_000_000_007;
const SIZE: usize = 26;

// Define a type alias for a 26x26 matrix
type Matrix = [[i64; SIZE]; SIZE];

/// Multiply two matrices modulo MOD.
///
/// This is the equivalent of the C++ function:
/// Matrix mul(Matrix& a, Matrix& b)
fn mul(a: &Matrix, b: &Matrix) -> Matrix {
    let mut c: Matrix = [[0; SIZE]; SIZE];
    // Loop over rows of a
    for i in 0..SIZE {
        // Loop over columns of a / rows of b
        for k in 0..SIZE {
            if a[i][k] == 0 {
                continue;
            }
            // Loop over columns of b
            for j in 0..SIZE {
                c[i][j] = (c[i][j] + a[i][k] * b[k][j]) % MOD;
            }
        }
    }
    c
}

/// Fast matrix exponentiation.
///
/// This is the equivalent of the C++ function:
/// Matrix pow(Matrix a, int n)
fn pow(mut a: Matrix, mut n: i32) -> Matrix {
    // Initialize res as the identity matrix
    let mut res: Matrix = [[0; SIZE]; SIZE];
    for i in 0..SIZE {
        res[i][i] = 1;
    }
    while n > 0 {
        if n & 1 == 1 {
            res = mul(&res, &a);
        }
        a = mul(&a, &a);
        n >>= 1;
    }
    res
}

/// Computes the length after transformations.
///
/// This corresponds to the C++ function:
/// int lengthAfterTransformations(string s, int t, vector<int>& nums)
fn length_after_transformations(s: &str, t: i32, nums: &[i64]) -> i64 {
    // Build the initial transformation matrix.
    // For each letter i, for j from i+1 to i+nums[i] (inclusive), set m[i][j%SIZE] = 1.
    let mut m: Matrix = [[0; SIZE]; SIZE];
    for i in 0..SIZE {
        let limit = nums[i] as usize;
        for j in (i + 1)..=(i + limit) {
            m[i][j % SIZE] = 1;
        }
    }
    // Raise the matrix to the power t.
    m = pow(m, t);

    // Count the occurrences of each letter in s.
    let mut cnt = [0i64; SIZE];
    for ch in s.chars() {
        // Only consider lowercase letters 'a' to 'z'.
        if ch >= 'a' && ch <= 'z' {
            cnt[(ch as u8 - b'a') as usize] += 1;
        }
    }

    // Calculate the final answer based on the matrix and frequency count.
    let mut ans = 0i64;
    for i in 0..SIZE {
        let row_sum: i64 = m[i].iter().fold(0, |acc, &x| (acc + x) % MOD);
        ans = (ans + row_sum * cnt[i]) % MOD;
    }
    ans
}

fn main() -> Result<(), Box<dyn Error>> {
    // Set up buffered reading from stdin.
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read tokens until we have at least three:
    // s_len, s, and t. The original C++ code does:
    //   cin >> s_len >> s >> t;
    let mut tokens: Vec<String> = Vec::new();
    while tokens.len() < 3 {
        if let Some(line) = lines.next() {
            let line = line?;
            tokens.extend(line.split_whitespace().map(String::from));
        } else {
            break;
        }
    }
    if tokens.len() < 3 {
        return Err("Insufficient input".into());
    }
    // The first token is s_len (which is ignored), the second is the string s,
    // and the third is the number t.
    let _s_len: usize = tokens[0].parse()?;
    let s = tokens[1].clone();
    let t: i32 = tokens[2].parse()?;

    // Now, read 26 values into the nums vector according to the original C++ input.
    let mut nums: Vec<i64> = Vec::with_capacity(SIZE);
    while nums.len() < SIZE {
        if let Some(line) = lines.next() {
            let line = line?;
            for token in line.split_whitespace() {
                if nums.len() < SIZE {
                    nums.push(token.parse()?);
                }
            }
        } else {
            break;
        }
    }
    if nums.len() != SIZE {
        return Err("Insufficient numbers for transformation".into());
    }

    // Compute the answer and print it.
    let ans = length_after_transformations(&s, t, &nums);
    println!("{}", ans);

    Ok(())
}