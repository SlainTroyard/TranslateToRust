use std::io::{self, BufRead, Write};

// We use i64 for all mod arithmetic
const MOD: i64 = 1_000_000_007;
const MX: usize = 100_000;

fn modpow(mut x: i64, mut n: i64) -> i64 {
    // fast exponentiation mod MOD
    let mut res = 1;
    while n > 0 {
        if n % 2 != 0 {
            res = res * x % MOD;
        }
        x = x * x % MOD;
        n /= 2;
    }
    res
}

// Precompute factorials and inverse factorials arrays for combination computation
struct Factorials {
    f: Vec<i64>,     // f[i] = i!
    inv_f: Vec<i64>, // inv_f[i] = (i!)^-1 mod MOD
}

impl Factorials {
    fn new() -> Self {
        // allocate vectors with size MX
        let mut f = vec![0i64; MX];
        let mut inv_f = vec![0i64; MX];
        f[0] = 1;
        // Precompute factorials: F[0] = 1, F[i] = F[i-1] * i % MOD for i >= 1
        for i in 1..MX {
            f[i] = f[i - 1] * (i as i64) % MOD;
        }
        // Compute the modular inverse for F[MX-1]
        inv_f[MX - 1] = modpow(f[MX - 1], MOD - 2);
        // Precompute inverse factorials: INV_F[i-1] = INV_F[i] * i % MOD for i from MX-1 down to 1.
        for i in (1..MX).rev() {
            inv_f[i - 1] = inv_f[i] * (i as i64) % MOD;
        }

        Factorials { f, inv_f }
    }

    // Function to compute combination C(n, m)
    // Returns 0 if m > n.
    fn comb(&self, n: usize, m: usize) -> i64 {
        if m > n {
            return 0;
        }
        // f[n] * inv_f[m] % MOD * inv_f[n-m] % MOD
        self.f[n] * self.inv_f[m] % MOD * self.inv_f[n - m] % MOD
    }
}

// Implement the solution for the problem.
struct Solution {
    factorials: Factorials,
}

impl Solution {
    fn new() -> Self {
        // Precompute factorials and inverse factorials at initialization
        let factorials = Factorials::new();
        Solution { factorials }
    }

    // Corresponds to minMaxSums(vector<int>& nums, int k) in C++
    fn min_max_sums(&self, mut nums: Vec<i64>, k: i32) -> i64 {
        // sort nums in non-decreasing order
        nums.sort_unstable();
        let n = nums.len();
        let mut ans: i64 = 0;
        let mut s: i64 = 1; // initial value s = 1

        for i in 0..n {
            // Add s * (nums[i] + nums[n - 1 - i]) to the answer modulo MOD
            ans = (ans + s * (nums[i] + nums[n - 1 - i]) % MOD) % MOD;
            // Update s = (s * 2 - comb(i, k-1) + MOD) % MOD
            // Note: In C++ code, comb(i, k-1) where i is int and k-1 is int.
            let comb_value = self.factorials.comb(i, (k - 1) as usize);
            s = (s * 2 % MOD - comb_value + MOD) % MOD;
        }

        ans
    }
}

fn main() -> io::Result<()> {
    // Set up buffered input and output
    let stdin = io::stdin();
    let mut reader = io::BufReader::new(stdin.lock());
    let stdout = io::stdout();
    let mut writer = io::BufWriter::new(stdout.lock());

    // Read the first line which contains n and k.
    // The input may have multiple values per line.
    let mut input_line = String::new();
    reader.read_line(&mut input_line)?;
    let mut tokens = input_line.split_whitespace();
    let n: usize = tokens
        .next()
        .expect("Expected n")
        .parse()
        .expect("n should be a number");
    let k: i32 = tokens
        .next()
        .expect("Expected k")
        .parse()
        .expect("k should be a number");

    // Read the next values until we have n numbers.
    let mut nums: Vec<i64> = Vec::with_capacity(n);
    while nums.len() < n {
        input_line.clear();
        reader.read_line(&mut input_line)?;
        // Allow multiple numbers per line.
        for token in input_line.split_whitespace() {
            if nums.len() < n {
                let num: i64 = token.parse().expect("number expected");
                nums.push(num);
            } else {
                break;
            }
        }
    }
    
    // Create a solution instance and compute the answer.
    let sol = Solution::new();
    let result = sol.min_max_sums(nums, k);

    // Write the result to stdout.
    writeln!(writer, "{}", result)?;
    writer.flush()?;
    Ok(())
}