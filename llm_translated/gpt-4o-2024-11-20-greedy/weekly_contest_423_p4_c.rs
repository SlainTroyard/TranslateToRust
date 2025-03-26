use std::io::{self, Read};

const MOST_CNT: usize = 801;
const MODULO_VAL: i32 = 1_000_000_007;

// Static global variables, equivalent to C's static globals.
static mut HAS_CALC: bool = false;
static mut DIGITS_CNT: [usize; MOST_CNT] = [0; MOST_CNT];
static mut REDUCIBLE_CNT: [usize; MOST_CNT] = [0; MOST_CNT];
static mut COMB_VAL: [[i32; MOST_CNT]; MOST_CNT] = [[0; MOST_CNT]; MOST_CNT];

/// Preprocessing function to calculate the number of 1's in binary representations,
/// reducible counts, and binomial coefficients.
fn pre_process() {
    unsafe {
        // Initialize `digitsCnt`, `reducibleCnt`, and `combVal`
        DIGITS_CNT[1] = 1;
        REDUCIBLE_CNT[1] = 0;
        COMB_VAL[0][0] = 1;
        COMB_VAL[1][0] = 1;
        COMB_VAL[1][1] = 1;

        for i in 2..MOST_CNT {
            // Count the number of 1's in binary representation
            DIGITS_CNT[i] = DIGITS_CNT[i >> 1] + (i & 1);
            // Calculate the reducibility based on the number of 1's
            REDUCIBLE_CNT[i] = REDUCIBLE_CNT[DIGITS_CNT[i]] + 1;

            // Calculate binomial coefficients
            COMB_VAL[i][0] = 1;
            COMB_VAL[i][i] = 1;
            for j in 1..i {
                COMB_VAL[i][j] = (COMB_VAL[i - 1][j] + COMB_VAL[i - 1][j - 1]) % MODULO_VAL;
            }
        }
    }
}

/// Translated function to calculate the count of K-reducible numbers.
fn count_k_reducible_numbers(s: &str, k: usize) -> i32 {
    let mut result = 0;

    unsafe {
        // Preprocess only once
        if !HAS_CALC {
            pre_process();
            HAS_CALC = true;
        }
    }

    let len = s.len();
    let mut one_count = s.chars().filter(|&ch| ch == '1').count();

    // Iterate through the string from right to left
    for (i, c) in s.chars().enumerate().rev() {
        if c == '1' {
            one_count -= 1;
            let j = len - i - 1;

            unsafe {
                for m in 0..=j {
                    if one_count + m > 0 && k > REDUCIBLE_CNT[one_count + m] {
                        result = (result + COMB_VAL[j][m]) % MODULO_VAL;
                    }
                }
            }
        }
    }

    result
}

fn main() {
    // Read from standard input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Parse input lines for s and k
    let mut lines = input.lines();
    let s = lines.next().unwrap().trim();
    let k: usize = lines.next().unwrap().trim().parse().unwrap();

    // Compute the result using the function
    let result = count_k_reducible_numbers(s, k);

    // Print the result as expected in the output format
    println!("{}", result);
}