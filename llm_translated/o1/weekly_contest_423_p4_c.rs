/// Weekly Contest 423 Problem 4 translated from C to Rust
/// 
/// Requirements met:
/// 1. The entire original file is translated, including preprocessing (global arrays),
///    the main function, and I/O handling.
/// 2. The logic and flow of the algorithm are preserved exactly.
/// 3. Uses idiomatic Rust with proper error handling and comments.
/// 4. Maintains the exact same stdin/stdout format as the original C code.
/// 5. Helpful comments are added to explain certain Rust-specific parts.

use std::io::{self, Write};
use std::sync::Once;

const MOST_CNT: usize = 801; 
const MODULO_VAL: i32 = 1000000007;

/// Global (static) arrays mirroring the C global arrays.
static mut DIGITS_CNT: [i32; MOST_CNT] = [0; MOST_CNT];
static mut REDUCIBLE_CNT: [i32; MOST_CNT] = [0; MOST_CNT];
static mut COMB_VAL: [[i32; MOST_CNT]; MOST_CNT] = [[0; MOST_CNT]; MOST_CNT];

/// A Once instance to ensure preprocessing is done only once.
static INIT: Once = Once::new();

/// Preprocessing function (run once).
unsafe fn pre_process() {
    // Sets the number of 1's (digitsCnt) for each number,
    // calculates how many steps to reduce each number to 1 (reducibleCnt),
    // and generates binomial coefficients (combVal).
    DIGITS_CNT[0] = 0;
    REDUCIBLE_CNT[0] = 0;
    DIGITS_CNT[1] = 1;
    REDUCIBLE_CNT[1] = 0;

    COMB_VAL[0][0] = 1;
    COMB_VAL[1][0] = 1;
    COMB_VAL[1][1] = 1;

    for i in 2..MOST_CNT {
        DIGITS_CNT[i] = DIGITS_CNT[i >> 1] + (i as i32 & 1);
        REDUCIBLE_CNT[i] = REDUCIBLE_CNT[DIGITS_CNT[i] as usize] + 1;
        COMB_VAL[i][0] = 1;
        COMB_VAL[i][i] = 1;

        for j in 1..i {
            COMB_VAL[i][j] = (COMB_VAL[i - 1][j] + COMB_VAL[i - 1][j - 1]) % MODULO_VAL;
        }
    }
}

/// Helper function to ensure we only pre-process once.
fn init_preprocess() {
    INIT.call_once(|| {
        unsafe {
            pre_process();
        }
    });
}

/// Translated version of the countKReducibleNumbers function from C.
fn count_k_reducible_numbers(s: &str, k: i32) -> i32 {
    // Ensure the global arrays are preprocessed exactly once.
    init_preprocess();

    // Count the total number of '1's and the length of the string.
    let mut one_count = 0_i32;
    for c in s.chars() {
        if c == '1' {
            one_count += 1;
        }
    }
    let len = s.len();

    let mut result = 0_i32;

    // Iterate through the bits from right to left.
    for i in (0..len).rev() {
        // If we find a '1', we consider flipping it to '0' and counting all
        // possible combinations of '1's in the remaining positions to the right.
        if s.as_bytes()[i] == b'1' {
            one_count -= 1; // We've accounted for this '1'.
            let j = (len - i - 1) as i32; // Bits to the right.

            // Try all possibilities of how many '1's go into those j bits.
            for m in 0..=j {
                // We only consider positive numbers, so one_count + m must be > 0.
                if one_count + m > 0 {
                    unsafe {
                        // Check if the number of steps to reduce (one_count + m) is < k.
                        if k > REDUCIBLE_CNT[(one_count + m) as usize] {
                            // Add the corresponding binomial coefficient.
                            result = (result + COMB_VAL[j as usize][m as usize]) % MODULO_VAL;
                        }
                    }
                }
            }
        }
    }
    result
}

/// Main function, matching the original I/O format exactly.
/// Reads a binary string on the first line, then an integer k on the second line.
/// Prints the result on a single line.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the binary string
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    // Trim to remove trailing newline and potential whitespace
    let s = s.trim().to_string();

    // Read the integer k
    let mut k_line = String::new();
    io::stdin().read_line(&mut k_line)?;
    let k: i32 = k_line.trim().parse()?;

    // Compute the result
    let result = count_k_reducible_numbers(&s, k);

    // Print the result with a newline, same as printf("%d\n", result).
    println!("{}", result);

    Ok(())
}