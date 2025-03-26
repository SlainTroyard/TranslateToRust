use lazy_static::lazy_static;
use std::io::Read;

const MOST_CNT: usize = 801;
const MODULO_VAL: i32 = 1_000_000_007;

struct PreprocessedData {
    digits_cnt: Vec<i32>,
    reducible_cnt: Vec<i32>,
    comb_val: Vec<Vec<i32>>,
}

lazy_static! {
    static ref PREPROCESSED: PreprocessedData = {
        let mut digits_cnt = vec![0; MOST_CNT];
        let mut reducible_cnt = vec![0; MOST_CNT];
        let mut comb_val = vec![vec![0; MOST_CNT]; MOST_CNT];

        // Initialize base cases
        comb_val[0][0] = 1;
        digits_cnt[0] = 0;
        reducible_cnt[0] = 0;

        if MOST_CNT > 1 {
            digits_cnt[1] = 1;
            reducible_cnt[1] = 0;
            comb_val[1][0] = 1;
            comb_val[1][1] = 1;
        }

        for i in 2..MOST_CNT {
            // Calculate number of 1's in binary representation
            digits_cnt[i] = digits_cnt[i >> 1] + (i & 1) as i32;
            // Calculate reduction steps
            reducible_cnt[i] = reducible_cnt[digits_cnt[i] as usize] + 1;
            // Precompute combinations using Pascal's triangle
            comb_val[i][0] = 1;
            comb_val[i][i] = 1;
            for j in 1..i {
                comb_val[i][j] = (comb_val[i - 1][j] + comb_val[i - 1][j - 1]) % MODULO_VAL;
            }
        }

        PreprocessedData {
            digits_cnt,
            reducible_cnt,
            comb_val,
        }
    };
}

fn count_k_reducible_numbers(s: &str, k: i32) -> i32 {
    let pre = &PREPROCESSED;
    let mut one_count = 0;
    let len = s.len();
    // Count total '1's in the binary string
    for c in s.chars() {
        if c == '1' {
            one_count += 1;
        }
    }

    let mut result = 0;
    let s_chars: Vec<char> = s.chars().collect();
    // Traverse from LSB to MSB
    for i in (0..len).rev() {
        if s_chars[i] == '1' {
            one_count -= 1;
            let j = len - i - 1; // Number of remaining bits to the right
            for m in 0..=j {
                let total_ones = one_count + m as i32;
                // Check if the number can be reduced in fewer than k steps
                if total_ones > 0 && pre.reducible_cnt[total_ones as usize] < k {
                    result = (result + pre.comb_val[j][m as usize]) % MODULO_VAL;
                }
            }
        }
    }
    result
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut parts = input.split_whitespace();
    let s = parts.next().expect("Missing binary string");
    let k: i32 = parts
        .next()
        .expect("Missing k value")
        .parse()
        .expect("Invalid k value");

    let result = count_k_reducible_numbers(s, k);
    println!("{}", result);
}