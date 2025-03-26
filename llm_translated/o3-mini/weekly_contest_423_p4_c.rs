use std::io::{self, BufRead, Write};
use std::sync::OnceLock;

// Constants used in the program.
const MOST_CNT: usize = 801;
const MODULO_VAL: i32 = 1_000_000_007;

// Structure to hold precomputed values.
struct Precomputed {
    // The number of 1's in the binary representation for values in the range [0, MOST_CNT-1].
    digits_cnt: [i32; MOST_CNT],
    // The number of steps needed to reduce each number to 1.
    reducible_cnt: [i32; MOST_CNT],
    // The binomial coefficients C(i, j) for 0 ≤ j ≤ i < MOST_CNT.
    comb_val: [[i32; MOST_CNT]; MOST_CNT],
}

// Use OnceLock to cache precomputed values so that this is done only once.
static PRECOMPUTED: OnceLock<Precomputed> = OnceLock::new();

/// Preprocess function that computes the digits count, reducible count,
/// and binomial coefficients. This function mirrors the logic in the original C code.
fn pre_process() -> Precomputed {
    let mut digits_cnt = [0i32; MOST_CNT];
    let mut reducible_cnt = [0i32; MOST_CNT];
    let mut comb_val = [[0i32; MOST_CNT]; MOST_CNT];

    // Initialize the base elements.
    digits_cnt[0] = 0;
    reducible_cnt[0] = 0;
    digits_cnt[1] = 1;
    reducible_cnt[1] = 0;
    comb_val[0][0] = 1;
    comb_val[1][0] = 1;
    comb_val[1][1] = 1;

    // Compute values for i in the range [2, MOST_CNT-1].
    for i in 2..MOST_CNT {
        // The number of 1's in the binary representation of i.
        // i >> 1 gives the value of i divided by 2
        // (i & 1) is 1 when the least significant bit is 1.
        digits_cnt[i] = digits_cnt[i >> 1] + (i as i32 & 1);
        // The reducible count of i is 1 more than that of its digit count.
        // (i.e. reducible_cnt[i] = reducible_cnt[digits_cnt[i]] + 1)
        let index = digits_cnt[i] as usize;
        reducible_cnt[i] = reducible_cnt[index] + 1;

        // Set the boundary combination values.
        comb_val[i][0] = 1;
        comb_val[i][i] = 1;
        // Calculate binomial coefficients using Pascal's triangle recurrence.
        for j in 1..i {
            comb_val[i][j] = (comb_val[i - 1][j] + comb_val[i - 1][j - 1]) % MODULO_VAL;
        }
    }

    Precomputed { digits_cnt, reducible_cnt, comb_val }
}

/// Computes the number of k-reducible numbers that are less than the binary number represented by s.
/// This function preserves the original logic of the C version exactly.
///
/// Parameters:
/// - s: A string slice representing a binary number.
/// - k: The reducibility limit.
///
/// Returns:
/// - The computed result as an i32.
fn count_k_reducible_numbers(s: &str, k: i32) -> i32 {
    // Ensure the precomputation is done only once.
    let precomputed = PRECOMPUTED.get_or_init(|| pre_process());

    // Count the total number of '1's in the string s.
    let mut one_count = s.chars().filter(|&ch| ch == '1').count() as i32;
    let len = s.len();
    let mut result = 0i32;

    // Process the string from right to left.
    for (i, ch) in s.char_indices().rev() {
        if ch == '1' {
            one_count -= 1;
            // j is the number of bits available to the right of the current bit.
            let j = (len - i - 1) as usize;
            // For each possible count m among the j positions,
            // combine with the one_count from left, and check if the total number of 1's
            // can be reduced to 1 in less than k steps.
            for m in 0..=j {
                let total_ones = one_count + m as i32;
                // Ensure total_ones is positive (problem requires positive integers)
                // and check the reducibility condition.
                if total_ones > 0 && k > precomputed.reducible_cnt[total_ones as usize] {
                    result = (result + precomputed.comb_val[j][m]) % MODULO_VAL;
                }
            }
        }
    }

    result
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Prepare to read input using a buffered reader.
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the binary string from input.
    let s = match lines.next() {
        Some(line) => line?,
        None => return Ok(()),
    };

    // Read the integer k from input.
    let k_line = match lines.next() {
        Some(line) => line?,
        None => return Ok(()),
    };
    let k: i32 = k_line.trim().parse()?;

    // Compute the result using the provided logic.
    let result = count_k_reducible_numbers(s.trim(), k);

    // Output the result. The output format exactly matches the C version.
    println!("{}", result);

    Ok(())
}