use std::io;
use std::str::FromStr;

const MODULO_VAL: i32 = 1000000007;
const RADIX_VAL: usize = 26;

#[derive(Clone, Copy)]
struct Matrix {
    m: [[i32; RADIX_VAL]; RADIX_VAL],
}

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

fn length_after_transformations(s: &str, t: i32, nums: &[i32]) -> i32 {
    let mut src = [0; RADIX_VAL];
    let mut init = Matrix { m: [[0; RADIX_VAL]; RADIX_VAL] };
    let mut dp = [Matrix { m: [[0; RADIX_VAL]; RADIX_VAL] }; 2];
    let mut digits = [0; 32];
    let mut digits_size = 0;

    // Initialize dp[0] to identity matrix and init matrix based on nums
    for x in 0..RADIX_VAL {
        dp[0].m[x][x] = 1;
        for y in 1..=nums[x] {
            let z = if RADIX_VAL > x + y { x + y } else { x + y - RADIX_VAL };
            init.m[z][x] = 1;
        }
    }

    // Count character frequencies in s
    for char_s in s.chars() {
        src[(char_s as u8 - b'a') as usize] += 1;
    }

    // Convert t to binary digits
    let mut temp_t = t;
    while temp_t != 0 {
        digits[digits_size] = temp_t & 1;
        digits_size += 1;
        temp_t >>= 1;
    }

    let mut z = 0;
    for x in (0..digits_size).rev() {
        let mut temp_dp = Matrix{m: [[0; RADIX_VAL]; RADIX_VAL]};
        matrix_multiply(&dp[z], &dp[z], &mut temp_dp);
        dp[1 - z] = temp_dp;

        if digits[x] == 1 {
            let mut temp_dp2 = Matrix{m: [[0; RADIX_VAL]; RADIX_VAL]};
            matrix_multiply(&dp[1 - z], &init, &mut temp_dp2);
            dp[z] = temp_dp2;
        } else {
            z = 1 - z;
        }
    }

    let mut result = 0;
    for x in 0..RADIX_VAL {
        for y in 0..RADIX_VAL {
            result = ((dp[z].m[x][y] as i64 * src[y] as i64 + result as i64) % MODULO_VAL as i64) as i32;
        }
    }
    result
}

fn main() {
    let mut s_len_str = String::new();
    io::stdin().read_line(&mut s_len_str).expect("Failed to read line");
    let s_len: usize = s_len_str.trim().parse().expect("Invalid input");

    let mut s_str = String::new();
    io::stdin().read_line(&mut s_str).expect("Failed to read line");
    let s = s_str.trim();

    let mut t_str = String::new();
    io::stdin().read_line(&mut t_str).expect("Failed to read line");
    let t: i32 = t_str.trim().parse().expect("Invalid input");

    let mut nums_str = String::new();
    io::stdin().read_line(&mut nums_str).expect("Failed to read line");
    let nums_vec: Vec<i32> = nums_str
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid input"))
        .collect();
    let nums: Vec<i32> = nums_vec;


    let result = length_after_transformations(s, t, &nums);
    println!("{}", result);
}