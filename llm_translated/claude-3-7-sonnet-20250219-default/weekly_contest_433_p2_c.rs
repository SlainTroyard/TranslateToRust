use std::io::{self, BufRead};

const MOD: i64 = 1_000_000_007;
const MX: usize = 100_000;

struct Solver {
    f: Vec<i64>,      // f[i] = i!
    inv_f: Vec<i64>,  // inv_f[i] = i!^-1
    initialized: bool,
}

impl Solver {
    fn new() -> Self {
        Solver {
            f: vec![0; MX],
            inv_f: vec![0; MX],
            initialized: false,
        }
    }

    // Power function for modular exponentiation
    fn power(&self, mut x: i64, mut n: i32) -> i64 {
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

    // Initialize factorial and inverse factorial arrays
    fn initialize(&mut self) {
        if self.initialized {
            return;
        }
        
        self.f[0] = 1;
        for i in 1..MX {
            self.f[i] = (self.f[i - 1] * i as i64) % MOD;
        }

        self.inv_f[MX - 1] = self.power(self.f[MX - 1], MOD as i32 - 2);
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

    // Main implementation function
    fn min_max_sums(&mut self, mut nums: Vec<i32>, k: usize) -> i32 {
        // Ensure factorial and inverse factorial arrays are initialized
        self.initialize();
        
        // Sort the array
        nums.sort();
        
        let nums_size = nums.len();
        let mut ans = 0;
        let mut s = 1;
        
        for i in 0..nums_size {
            ans = (ans + s * (nums[i] as i64 + nums[nums_size - 1 - i] as i64)) % MOD;
            s = (s * 2 - self.comb(i, k - 1) + MOD) % MOD;
        }
        
        ans as i32
    }
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
    let nums: Vec<i32> = second_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Check that we got the expected number of values
    assert_eq!(nums.len(), n, "Expected {} numbers in the input", n);
    
    // Calculate the result
    let mut solver = Solver::new();
    let result = solver.min_max_sums(nums, k);
    
    // Output the result
    println!("{}", result);
    
    Ok(())
}