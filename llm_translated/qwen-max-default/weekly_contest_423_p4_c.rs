use std::io;

const MOST_CNT: usize = 801;
const MODULO_VAL: i64 = 1_000_000_007;

static mut HAS_CALC: i32 = 0;
static mut DIGITS_CNT: [i32; MOST_CNT] = [0; MOST_CNT];
static mut REDUCIBLE_CNT: [i32; MOST_CNT] = [0; MOST_CNT];
static mut COMB_VAL: [[i64; MOST_CNT]; MOST_CNT] = [[0; MOST_CNT]; MOST_CNT];

fn pre_process() {
    unsafe {
        if HAS_CALC == 1 {
            return;
        }

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
                COMB_VAL[i][j] = (COMB_VAL[i - 1][j] + COMB_VAL[i - 1][j - 1]) % MODULO_VAL;
            }
        }

        HAS_CALC = 1;
    }
}

fn count_k_reducible_numbers(s: &str, k: i32) -> i64 {
    let mut result = 0;
    let mut one = 0;
    let len = s.len();

    // Count the total number of 1's in the string
    for c in s.chars() {
        if c == '1' {
            one += 1;
        }
    }

    // Check from right to left
    for i in (0..len).rev() {
        if s.as_bytes()[i] == b'1' {
            one -= 1;
            let j = len - i - 1;
            for m in 0..=j {
                let sum = one + m;
                if sum > 0 && k > unsafe { REDUCIBLE_CNT[sum as usize] } {
                    result = (result + unsafe { COMB_VAL[j][m] }) % MODULO_VAL;
                }
            }
        }
    }

    result
}

fn main() {
    // Read the input string and the integer k
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    s = s.trim().to_string();

    let mut k = String::new();
    io::stdin().read_line(&mut k).expect("Failed to read line");
    let k: i32 = k.trim().parse().expect("Please type a number!");

    // Preprocess
    pre_process();

    // Call the function to compute the result
    let result = count_k_reducible_numbers(&s, k);

    // Output the result
    println!("{}", result);
}