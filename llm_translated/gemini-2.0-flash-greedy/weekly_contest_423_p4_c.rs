use std::io;
use std::str::FromStr;

const MOST_CNT: usize = 801;
const MODULO_VAL: i32 = 1_000_000_007;

// Global variables are used to ensure that the preprocessing only needs to be done once for all test cases.
// hasCalc:        Used to check if preprocessing has been executed. After execution, it is set to 1.
// digitsCnt:      The number of 1's in the binary representation of values in the range [0, 800].
// reducibleCnt:   The number of steps needed for each number to be reduced to 1.
// combVal:        The binomial coefficients C(m,n), where C(m,n) = C(m-1,n) + C(m-1,n-1).
static mut HAS_CALC: i32 = 0;
static mut DIGITS_CNT: [i32; MOST_CNT] = [0; MOST_CNT];
static mut REDUCIBLE_CNT: [i32; MOST_CNT] = [0; MOST_CNT];
static mut COMB_VAL: [[i32; MOST_CNT]; MOST_CNT] = [[0; MOST_CNT]; MOST_CNT];

// Preprocessing function.
fn pre_process() {
    unsafe {
        // Calculate the number of 1's in each value and their reducibility count, as well as the combination numbers C(i, j).
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

// Main function.
fn count_k_reducible_numbers(s: &str, k: i32) -> i32 {
    let mut one = 0;
    let len = s.len();

    // Preprocess. After processing, set hasCalc to 1 so no need to repeat the preprocessing.
    unsafe {
        if HAS_CALC == 0 {
            pre_process();
            HAS_CALC = 1;
        }
    }

    // Count the total number of 1's in the string.
    for c in s.chars() {
        if c == '1' {
            one += 1;
        }
    }

    let mut result = 0;
    let s_bytes = s.as_bytes();

    // Check from right to left. When a bit is 1, changing it to 0 and freely combining the remaining bits will result in a number smaller than n.
    for i in (0..len).rev() {
        if s_bytes[i] as char == '1' {
            one -= 1;
            // Now, there are 'one' number of 1's to the left, this bit becomes 0, and there are 'j' bits available to the right, which means there could be [0, j] 1's.
            let j = len - i - 1;
            for m in 0..=j {
                // The "one + m" 1's need to be reduced to "one + m", which should be greater than 0 (the problem requires positive integers, so it can't be 0).
                unsafe {
                    if one + m as i32 > 0 && k > REDUCIBLE_CNT[(one + m as i32) as usize] {
                        result = (result + COMB_VAL[j][m]) % MODULO_VAL;
                    }
                }
            }
        }
    }
    result
}

fn main() -> io::Result<()> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let s = s.trim();

    let mut k_str = String::new();
    io::stdin().read_line(&mut k_str)?;
    let k = i32::from_str(k_str.trim()).unwrap();

    let result = count_k_reducible_numbers(s, k);

    println!("{}", result);

    Ok(())
}