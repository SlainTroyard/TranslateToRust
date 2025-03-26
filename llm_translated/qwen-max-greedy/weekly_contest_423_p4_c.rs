use std::io;

const MOST_CNT: usize = 801;
const MODULO_VAL: i64 = 1_000_000_007;

// Global variables are used to ensure that the preprocessing only needs to be done once for all test cases.
static mut HAS_CALC: i32 = 0;
static mut DIGITS_CNT: [i32; MOST_CNT] = [0; MOST_CNT];
static mut REDUCIBLE_CNT: [i32; MOST_CNT] = [0; MOST_CNT];
static mut COMB_VAL: [[i32; MOST_CNT]; MOST_CNT] = [[0; MOST_CNT]; MOST_CNT];

/// Preprocessing function.
fn pre_process() {
    unsafe {
        HAS_CALC = 1;
        DIGITS_CNT[0] = 0;
        REDUCIBLE_CNT[0] = 0;
        DIGITS_CNT[1] = 1;
        REDUCIBLE_CNT[1] = 0;
        COMB_VAL[0][0] = 1;
        COMB_VAL[1][0] = 1;
        COMB_VAL[1][1] = 1;

        for i in 2..MOST_CNT {
            DIGITS_CNT[i] = DIGITS_CNT[i >> 1] + (i & 1);
            REDUCIBLE_CNT[i] = REDUCIBLE_CNT[DIGITS_CNT[i] as usize] + 1;
            COMB_VAL[i][0] = 1;
            COMB_VAL[i][i] = 1;
            for j in 1..i {
                COMB_VAL[i][j] = (COMB_VAL[i - 1][j] + COMB_VAL[i - 1][j - 1]) % MODULO_VAL as i32;
            }
        }
    }
}

/// Main function to count k-reducible numbers.
fn count_k_reducible_numbers(s: &str, k: i32) -> i64 {
    let mut one = 0;
    let len = s.len();
    let mut result = 0;

    // Count the total number of 1's in the string and calculate its length.
    for c in s.chars() {
        if c == '1' {
            one += 1;
        }
    }

    // Check from right to left. When a bit is 1, changing it to 0 and freely combining the remaining bits will result in a number smaller than n.
    for i in (0..len).rev() {
        if s.as_bytes()[i] == b'1' {
            one -= 1;
            let j = len - i - 1;
            for m in 0..=j {
                // The "one + m" 1's need to be reduced to "one + m", which should be greater than 0 (the problem requires positive integers, so it can't be 0).
                if 0 < one + m && k > unsafe { REDUCIBLE_CNT[one + m as usize] } {
                    result = (result + unsafe { COMB_VAL[j][m] as i64 }) % MODULO_VAL;
                }
            }
        }
    }

    result
}

fn main() {
    // Read the input string and the integer k.
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    let s = s.trim();

    let mut k = String::new();
    io::stdin().read_line(&mut k).expect("Failed to read line");
    let k: i32 = k.trim().parse().expect("Please type a number!");

    // Preprocess. After processing, set hasCalc to 1 so no need to repeat the preprocessing.
    unsafe {
        if HAS_CALC == 0 {
            pre_process();
        }
    }

    // Call the function to compute the result
    let result = count_k_reducible_numbers(s, k);

    // Output the result
    println!("{}", result);
}