use std::io::{self, BufRead};

// We use 64-bit integers for safe multiplication before taking modulo.
const MOD: i64 = 1_000_000_007;
const SIZE: usize = 26;

// Define a 26x26 matrix type.
type Matrix = [[i64; SIZE]; SIZE];

fn main() -> io::Result<()> {
    // Read all tokens (whitespace-separated) from stdin.
    let tokens = read_tokens()?;
    let mut tokens_iter = tokens.into_iter();

    // Parse input in the same order as the C++ code:
    // 1) s_len (not actually used, but we must consume it to match input format)
    // 2) s
    // 3) t
    // 4) The 26 integers for nums
    let _s_len = parse_i32(&mut tokens_iter);
    let s = parse_string(&mut tokens_iter);
    let t = parse_i32(&mut tokens_iter);

    let mut nums = vec![0; SIZE];
    for i in 0..SIZE {
        nums[i] = parse_i32(&mut tokens_iter);
    }

    // Compute and print the answer
    let ans = length_after_transformations(&s, t, &nums);
    println!("{}", ans);

    Ok(())
}

// Reads all lines from stdin, splits by whitespace, and collects into a Vec<String>.
fn read_tokens() -> io::Result<Vec<String>> {
    let stdin = io::stdin();
    let mut tokens = vec![];
    for line in stdin.lock().lines() {
        let line = line?;
        tokens.extend(line.split_whitespace().map(|s| s.to_string()));
    }
    Ok(tokens)
}

// Parses the next token as i32.
fn parse_i32(iter: &mut impl Iterator<Item = String>) -> i32 {
    iter.next().unwrap().parse().unwrap()
}

// Parses the next token as String.
fn parse_string(iter: &mut impl Iterator<Item = String>) -> String {
    iter.next().unwrap()
}

// Translates the core solution logic from C++ to Rust.
fn length_after_transformations(s: &str, t: i32, nums: &[i32]) -> i64 {
    // Build the transition matrix 'm' in the same way as the C++ code.
    let mut m = [[0i64; SIZE]; SIZE];
    for i in 0..SIZE {
        let i_i32 = i as i32;
        // From i+1 to i+nums[i], we set m[i][(j % SIZE)] = 1
        for j in (i_i32 + 1)..=(i_i32 + nums[i]) {
            m[i][(j as usize) % SIZE] = 1;
        }
    }

    // Raise the matrix to the power t.
    let m = matrix_pow(m, t);

    // Count how many times each character appears in the input string.
    let mut cnt = [0i64; SIZE];
    for c in s.chars() {
        let idx = (c as u8 - b'a') as usize;
        cnt[idx] += 1;
    }

    // Sum up the contributions for each row.
    let mut ans = 0i64;
    for i in 0..SIZE {
        let row_sum: i64 = m[i].iter().sum();
        ans += row_sum * cnt[i];
    }
    ans % MOD
}

// Matrix multiplication under modulo.
fn matrix_mul(a: &Matrix, b: &Matrix) -> Matrix {
    let mut c = [[0; SIZE]; SIZE];
    for i in 0..SIZE {
        for k in 0..SIZE {
            // Skip multiplying by zero, same as in the C++ code for efficiency.
            if a[i][k] != 0 {
                let av = a[i][k];
                for j in 0..SIZE {
                    let bv = b[k][j];
                    let val = (av * bv) % MOD;
                    c[i][j] = (c[i][j] + val) % MOD;
                }
            }
        }
    }
    c
}

// Fast exponentiation of a 26x26 matrix by an integer exponent n.
fn matrix_pow(mut a: Matrix, mut n: i32) -> Matrix {
    // Initialize result as the identity matrix.
    let mut res = [[0; SIZE]; SIZE];
    for i in 0..SIZE {
        res[i][i] = 1;
    }

    // Exponentiate by squaring.
    while n > 0 {
        if n & 1 == 1 {
            res = matrix_mul(&res, &a);
        }
        a = matrix_mul(&a, &a);
        n >>= 1;
    }
    res
}