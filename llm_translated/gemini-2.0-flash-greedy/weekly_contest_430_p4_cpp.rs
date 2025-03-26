use std::io;

const MOD: i64 = 1_000_000_007;

struct Solution {}

impl Solution {
    fn exponent(&self, base: i64, exp: i64, mod_val: i64) -> i64 {
        let mut result: i64 = 1;
        let mut base_mut = base;
        let mut exp_mut = exp;
        while exp_mut > 0 {
            if exp_mut % 2 == 1 {
                result = (result * base_mut) % mod_val;
            }
            base_mut = (base_mut * base_mut) % mod_val;
            exp_mut /= 2;
        }
        result
    }

    fn mod_inverse(&self, x: i64, mod_val: i64) -> i64 {
        self.exponent(x, mod_val - 2, mod_val)
    }

    fn ncr(&self, n: usize, r: usize, fact: &Vec<i64>) -> i64 {
        fact[n] * self.mod_inverse(fact[r], MOD) % MOD * self.mod_inverse(fact[n - r], MOD) % MOD
    }

    fn count_good_arrays(&self, n: usize, m: i64, k: usize) -> i64 {
        let mut fact: Vec<i64> = vec![1; n + 1];
        for i in 2..=n {
            fact[i] = fact[i - 1] * i as i64 % MOD;
        }

        let mut result = self.ncr(n - 1, k, &fact);
        result = result * m % MOD;
        result = result * self.exponent(m - 1, (n - k - 1) as i64, MOD) % MOD;
        result
    }
}

fn main() -> io::Result<()> {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line)?;
    let values: Vec<i32> = input_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let n = values[0] as usize;
    let m = values[1] as i64;
    let k = values[2] as usize;

    let solution = Solution {};
    let result = solution.count_good_arrays(n, m, k);

    println!("{}", result);

    Ok(())
}