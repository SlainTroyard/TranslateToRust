const MODULO_VAL: i32 = 1_000_000_007;
const MOST_CNT: usize = 801;

static mut HAS_CALC: bool = false;
static mut DIGITS_CNT: [i32; MOST_CNT] = [0; MOST_CNT];
static mut REDUCIBLE_CNT: [i32; MOST_CNT] = [0; MOST_CNT];
static mut COMB_VAL: [[i32; MOST_CNT]; MOST_CNT] = [[0; MOST_CNT]; MOST_CNT];

// Preprocess the static arrays once before any calculations
unsafe fn pre_process() {
    // Initialize base cases for digits count and reducible count
    DIGITS_CNT[0] = 0;
    REDUCIBLE_CNT[0] = 0;
    DIGITS_CNT[1] = 1;
    REDUCIBLE_CNT[1] = 0;

    // Initialize binomial coefficients base cases
    COMB_VAL[0][0] = 1;
    COMB_VAL[1][0] = 1;
    COMB_VAL[1][1] = 1;

    // Fill the arrays for values from 2 to MOST_CNT-1
    for i in 2..MOST_CNT {
        // Calculate number of 1's in binary representation of i
        let half = i >> 1;
        let bit = i & 1;
        DIGITS_CNT[i] = DIGITS_CNT[half] + (bit as i32);

        // Calculate reducible steps based on the count of 1's
        let d = DIGITS_CNT[i] as usize;
        REDUCIBLE_CNT[i] = REDUCIBLE_CNT[d] + 1;

        // Initialize binomial coefficients for current row
        COMB_VAL[i][0] = 1;
        COMB_VAL[i][i] = 1;

        // Compute binomial coefficients using dynamic programming
        for j in 1..i {
            let prev_row = i - 1;
            COMB_VAL[i][j] = (COMB_VAL[prev_row][j] + COMB_VAL[prev_row][j - 1]) % MODULO_VAL;
        }
    }
}

// Main logic to count valid numbers
fn count_k_reducible_numbers(s: &str, k: i32) -> i32 {
    unsafe {
        if !HAS_CALC {
            pre_process();
            HAS_CALC = true;
        }
    }

    let s_chars: Vec<char> = s.chars().collect();
    let len = s_chars.len();
    let mut one_count = 0;

    // Count total number of '1's in the string
    for &c in &s_chars {
        if c == '1' {
            one_count += 1;
        }
    }

    let mut result = 0;

    // Iterate from the end of the string to the start
    for i in (0..len).rev() {
        let current_char = s_chars[i];
        if current_char == '1' {
            one_count -= 1; // Exclude the current '1' since we're considering flipping it to '0'
            let j = len - i - 1; // Number of bits to the right of current position

            // Try all possible combinations of '1's in the right part
            for m in 0..=j {
                let total_ones = one_count + m;
                unsafe {
                    if total_ones > 0 && k > REDUCIBLE_CNT[total_ones as usize] {
                        let comb = COMB_VAL[j][m];
                        result = (result + comb) % MODULO_VAL;
                    }
                }
            }
        }
    }

    result
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)
        .expect("Failed to read input");
    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.len() < 2 {
        panic!("Input must contain a binary string followed by an integer");
    }

    let s = parts[0];
    let k: i32 = parts[1].parse().expect("Second argument must be an integer");

    let result = count_k_reducible_numbers(s, k);
    println!("{}", result);
}