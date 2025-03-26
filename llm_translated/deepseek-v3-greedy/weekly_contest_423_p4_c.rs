use std::io::{self, Write};

const MOST_CNT: usize = 801;
const MODULO_VAL: i32 = 1_000_000_007;

static mut HAS_CALC: bool = false;
static mut DIGITS_CNT: [i32; MOST_CNT] = [0; MOST_CNT];
static mut REDUCIBLE_CNT: [i32; MOST_CNT] = [0; MOST_CNT];
static mut COMB_VAL: [[i32; MOST_CNT]; MOST_CNT] = [[0; MOST_CNT]; MOST_CNT];

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

fn count_k_reducible_numbers(s: &str, k: i32) -> i32 {
    unsafe {
        if !HAS_CALC {
            pre_process();
            HAS_CALC = true;
        }

        let mut one = 0;
        let len = s.len();

        for c in s.chars() {
            if c == '1' {
                one += 1;
            }
        }

        let mut result = 0;
        let chars: Vec<char> = s.chars().collect();

        for i in (0..len).rev() {
            if chars[i] == '1' {
                one -= 1;
                let j = len - i - 1;

                for m in 0..=j {
                    if one + m > 0 && k > REDUCIBLE_CNT[(one + m) as usize] {
                        result = (result + COMB_VAL[j][m]) % MODULO_VAL;
                    }
                }
            }
        }

        result
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let s = input.trim();

    let mut k_input = String::new();
    io::stdin().read_line(&mut k_input).expect("Failed to read input");
    let k: i32 = k_input.trim().parse().expect("Invalid input for k");

    let result = count_k_reducible_numbers(s, k);
    println!("{}", result);
}