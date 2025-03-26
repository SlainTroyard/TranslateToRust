use std::io::{self, BufRead};

const MOST_CNT: usize = 801;
const MODULO_VAL: i64 = 1_000_000_007;

// Global variables to ensure preprocessing is done only once for all test cases.
static mut HAS_CALC: bool = false;
static mut DIGITS_CNT: [i32; MOST_CNT] = [0; MOST_CNT];
static mut REDUCIBLE_CNT: [i32; MOST_CNT] = [0; MOST_CNT];
static mut COMB_VAL: [[i64; MOST_CNT]; MOST_CNT] = [[0; MOST_CNT]; MOST_CNT];

// Preprocessing function.
fn pre_process() {
    unsafe {
        DIGITS_CNT[0] = 0;
        REDUCIBLE_CNT[0] = 0;
        DIGITS_CNT[1] = 1;
        REDUCIBLE_CNT[1] = 0;
        COMB_VAL[0][0] = 1;
        COMB_VAL[1][0] = 1;
        COMB_VAL[1][1] = 1;
        for i in 2..MOST_CNT {
            DIGITS_CNT[i] = DIGITS_CNT[i >> 1] + (i & 1) as i32;
            REDUCIBLE_CNT[i] = REDUCIBLE_CNT[DIGITS_CNT[i] as usize] + 1;
            COMB_VAL[i][0] = 1;
            COMB_VAL[i][i] = 1;
            for j in 1..i {
                COMB_VAL[i][j] = (COMB_VAL[i - 1][j] + COMB_VAL[i - 1][j - 1]) % MODULO_VAL;
            }
        }
    }
}

// Main function to count k-reducible numbers.
fn count_k_reducible_numbers(s: &str, k: i32) -> i64 {
    unsafe {
        if !HAS_CALC {
            pre_process();
            HAS_CALC = true;
        }
    }

    let mut one = 0;
    let mut result = 0;
    let len = s.len();

    // Count the total number of 1's in the string.
    for c in s.chars() {
        if c == '1' {
            one += 1;
        }
    }

    // Check from right to left. When a bit is 1, changing it to 0 and freely combining the remaining bits will result in a number smaller than n.
    for i in (0..len).rev() {
        if s.chars().nth(i).unwrap() == '1' {
            one -= 1;
            let j = len - i - 1;
            for m in 0..=j {
                // The "one + m" 1's need to be reduced to "one + m", which should be greater than 0 (the problem requires positive integers, so it can't be 0).
                if one + m as i32 > 0 && k > unsafe { REDUCIBLE_CNT[(one + m as i32) as usize] } {
                    result = (result + unsafe { COMB_VAL[j][m] }) % MODULO_VAL;
                }
            }
        }
    }

    result
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the input string and the integer k.
    let s = lines.next().unwrap()?;
    let k: i32 = lines.next().unwrap()?.parse().unwrap();

    // Call the function to compute the result
    let result = count_k_reducible_numbers(&s, k);

    // Output the result
    println!("{}", result);

    Ok(())
}