// Problem: Weekly Contest 423 Problem 4
// Translated from C to Rust

const MOST_CNT: usize = 801;
const MODULO_VAL: i32 = 1000000007;

// Precomputed values for digits count, reducibility count, and binomial coefficients.
static DIGITS_CNT: [i32; MOST_CNT] = {
    let mut digits_cnt = [0; MOST_CNT];
    digits_cnt[1] = 1;
    for i in 2..MOST_CNT {
        digits_cnt[i] = digits_cnt[i >> 1] + (i & 1) as i32;
    }
    digits_cnt
};

static REDUCIBLE_CNT: [i32; MOST_CNT] = {
    let mut reducible_cnt = [0; MOST_CNT];
    reducible_cnt[1] = 0;
    for i in 2..MOST_CNT {
        reducible_cnt[i] = reducible_cnt[DIGITS_CNT[i] as usize] + 1;
    }
    reducible_cnt
};

static COMB_VAL: [[i32; MOST_CNT]; MOST_CNT] = {
    let mut comb_val = [[0; MOST_CNT]; MOST_CNT];
    comb_val[0][0] = 1;
    comb_val[1][0] = 1;
    comb_val[1][1] = 1;
    for i in 2..MOST_CNT {
        comb_val[i][0] = 1;
        comb_val[i][i] = 1;
        for j in 1..i {
            comb_val[i][j] = (comb_val[i - 1][j] + comb_val[i - 1][j - 1]) % MODULO_VAL;
        }
    }
    comb_val
};

fn count_k_reducible_numbers(s: &str, k: i32) -> i32 {
    let mut one_count = 0;
    for char_s in s.chars() {
        if char_s == '1' {
            one_count += 1;
        }
    }
    let len = s.len();
    let mut result = 0;

    for i in 0..len {
        if s.chars().nth(i).unwrap() == '1' {
            one_count -= 1;
            let j = len - i - 1;
            for m in 0..=j {
                if one_count + m > 0 && k > REDUCIBLE_CNT[(one_count + m) as usize] {
                    result = (result + COMB_VAL[j][m]) % MODULO_VAL;
                }
            }
        }
    }
    result
}

fn main() {
    use std::io;

    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    let s = s.trim();

    let mut k_str = String::new();
    io::stdin().read_line(&mut k_str).expect("Failed to read line");
    let k: i32 = k_str.trim().parse().expect("Failed to parse k");

    let result = count_k_reducible_numbers(s, k);
    println!("{}", result);
}