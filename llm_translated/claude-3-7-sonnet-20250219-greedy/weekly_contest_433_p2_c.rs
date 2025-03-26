use std::io::{self, BufRead};

const MOD: i64 = 1_000_000_007;
const MX: usize = 100_000;

// Global state for factorial and inverse factorial
struct MathState {
    f: Vec<i64>,      // F[i] = i!
    inv_f: Vec<i64>,  // INV_F[i] = i!^-1
    initialized: bool,
}

impl MathState {
    fn new() -> Self {
        MathState {
            f: vec![0; MX],
            inv_f: vec![0; MX],
            initialized: false,
        }
    }

    // Initialize factorial and inverse factorial arrays
    fn initialize(&mut self) {
        if self.initialized {
            return;
        }

        self.f[0] = 1;
        for i in 1..MX {
            self.f[i] = (self.f[i - 1] * i as i64) % MOD;
        }

        self.inv_f[MX - 1] = power(self.f[MX - 1], MOD - 2);
        for i in (1..MX).rev() {
            self.inv_f[i - 1] = (self.inv_f[i] * i as i64) % MOD;
        }

        self.initialized = true;
    }

    // Combination calculation function
    fn comb(&self, n: usize, m: usize) -> i64 {
        if m > n {
            return 0;
        }
        (((self.f[n] * self.inv_f[m]) % MOD) * self.inv_f[n - m]) % MOD
    }
}

// Power function for modular exponentiation
fn power(mut x: i64, mut n: i64) -> i64 {
    let mut res = 1;
    while n > 0 {
        if n % 2 == 1 {
            res = (res * x) % MOD;
        }
        x = (x * x) % MOD;
        n /= 2;
    }
    res
}

// Main function implementation
fn min_max_sums(nums: &mut [i32], k: usize, math_state: &mut MathState) -> i32 {
    // Ensure factorial and inverse factorial arrays are initialized
    math_state.initialize();
    
    // Sort the array
    nums.sort();
    
    let mut ans = 0;
    let mut s = 1;
    let n = nums.len();
    
    for i in 0..n {
        ans = (ans + s * (nums[i] as i64 + nums[n - 1 - i] as i64)) % MOD;
        s = (s * 2 - math_state.comb(i, k - 1) + MOD) % MOD;
    }
    
    ans as i32
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read n and k
    let first_line = lines.next().unwrap()?;
    let parts: Vec<usize> = first_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let n = parts[0];
    let k = parts[1];
    
    // Read the nums array
    let second_line = lines.next().unwrap()?;
    let mut nums: Vec<i32> = second_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Initialize math state
    let mut math_state = MathState::new();
    
    // Calculate and print the result
    let result = min_max_sums(&mut nums, k, &mut math_state);
    println!("{}", result);
    
    Ok(())
}