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
        let n = n as usize;
        let r = r as usize;
        (fact[n] * Self::mod_inverse(fact[r], MOD) % MOD * Self::mod_inverse(fact[n - r], MOD)) % MOD
    }

    fn count_good_arrays(n: i32, m: i32, k: i32) -> i32 {
        let mut fact = vec![1; (n + 1) as usize];
        for i in 2..=n as usize {
            fact[i] = (fact[i - 1] * i as i64) % MOD;
        }

        let mut result = Self::n_cr(n - 1, k, &fact);
        result = (result * m as i64) % MOD;
        result = (result * Self::exponent((m - 1) as i64, (n - k - 1) as i64, MOD)) % MOD;
        
        result as i32
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    let values: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse input"))
        .collect();
    
    let n = values[0];
    let m = values[1];
    let k = values[2];

    let solution = Solution;
    let result = solution.count_good_arrays(n, m, k);

    println!("{}", result);

    Ok(())
}