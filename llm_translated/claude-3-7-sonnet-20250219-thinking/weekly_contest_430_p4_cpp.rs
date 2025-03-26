use std::io;

const MOD: i64 = 1_000_000_007;

struct Solution;

impl Solution {
    fn exponent(mut base: i64, mut exp: i64, mod_val: i64) -> i64 {
        let mut result = 1;
        while exp > 0 {
            if exp % 2 == 1 {
                result = (result * base) % mod_val;
            }
            base = (base * base) % mod_val;
            exp /= 2;
        }
        result
    }

    fn mod_inverse(x: i64, mod_val: i64) -> i64 {
        Self::exponent(x, mod_val - 2, mod_val)
    }

    fn n_cr(n: i32, r: i32, fact: &[i64]) -> i64 {
        fact[n as usize] * Self::mod_inverse(fact[r as usize], MOD) % MOD 
            * Self::mod_inverse(fact[(n - r) as usize], MOD) % MOD
    }

    fn count_good_arrays(n: i32, m: i32, k: i32) -> i32 {
        let mut fact = vec![1i64; (n + 1) as usize];
        for i in 2..=n {
            fact[i as usize] = fact[(i - 1) as usize] * i as i64 % MOD;
        }

        let mut result = Self::n_cr(n - 1, k, &fact);
        result = result * m as i64 % MOD;
        result = result * Self::exponent((m - 1) as i64, (n - k - 1) as i64, MOD) % MOD;
        result as i32
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    let parts: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Expected an integer"))
        .collect();
    
    let n = parts[0];
    let m = parts[1];
    let k = parts[2];

    let solution = Solution;
    let result = solution.count_good_arrays(n, m, k);

    println!("{}", result);
}