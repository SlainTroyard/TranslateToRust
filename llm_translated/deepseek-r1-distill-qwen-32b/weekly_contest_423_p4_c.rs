use lazy_static::lazy_static;
use std::io;

const MOST_CNT: usize = 801;
const MODULO: i32 = 1_000_000_007;

lazy_static! {
    static ref DIGITS_CNT: Vec<i32> = {
        let mut v = vec![0; MOST_CNT];
        v[0] = 0;
        v[1] = 1;
        for i in 2..MOST_CNT {
            v[i] = v[i >> 1] + (i & 1) as i32;
        }
        v
    };

    static ref REDUCIBLE_CNT: Vec<i32> = {
        let mut v = vec![0; MOST_CNT];
        v[0] = 0;
        v[1] = 0;
        for i in 2..MOST_CNT {
            v[i] = v[DIGITS_CNT[i] as usize] + 1;
        }
        v
    };

    static ref COMB_VAL: Vec<Vec<i32>> = {
        let mut v = vec![vec![0; MOST_CNT]; MOST_CNT];
        v[0][0] = 1;
        if MOST_CNT > 1 {
            v[1][0] = 1;
            v[1][1] = 1;
        }
        for i in 2..MOST_CNT {
            v[i][0] = 1;
            v[i][i] = 1;
            for j in 1..i {
                v[i][j] = (v[i-1][j] + v[i-1][j-1]) % MODULO;
            }
        }
        v
    };
}

fn count_k_reducible_numbers(s: &str, k: i32) -> i32 {
    let len = s.len();
    let mut one = 0;
    for c in s.chars() {
        if c == '1' {
            one += 1;
        }
    }

    let mut result = 0;
    for i in (0..len).rev() {
        let c = s.chars().nth(i).unwrap();
        if c == '1' {
            one -= 1;
            let j = len - i - 1;
            for m in 0..=j {
                let total_ones = one + m;
                if total_ones > 0 && k > REDUCIBLE_CNT[total_ones as usize] {
                    result = (result + COMB_VAL[j][m]) % MODULO;
                }
            }
        }
    }
    result
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let parts: Vec<&str> = input.split_whitespace().collect();
    let s = parts[0];
    let k = parts[1].parse::<i32>().unwrap();

    let result = count_k_reducible_numbers(s, k);
    println!("{}", result);
}