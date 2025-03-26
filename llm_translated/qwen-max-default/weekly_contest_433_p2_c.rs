use std::io::{self, BufRead, Write};

const MOD: i64 = 1_000_000_007;
const MX: usize = 100_000;

// Precomputed factorials and their modular inverses
static mut F: [i64; MX] = [0; MX];
static mut INV_F: [i64; MX] = [0; MX];
static mut INITIALIZED: bool = false;

fn initialize() {
    unsafe {
        F[0] = 1;
        for i in 1..MX {
            F[i] = (F[i - 1] * i as i64) % MOD;
        }

        INV_F[MX - 1] = power(F[MX - 1], MOD - 2);
        for i in (1..MX).rev() {
            INV_F[i - 1] = (INV_F[i] * i as i64) % MOD;
        }
        INITIALIZED = true;
    }
}

fn power(mut x: i64, mut n: i32) -> i64 {
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

fn comb(n: usize, m: usize) -> i64 {
    if m > n {
        return 0;
    }
    unsafe {
        (((F[n] * INV_F[m]) % MOD) * INV_F[n - m]) % MOD
    }
}

fn min_max_sums(nums: &mut [i32], k: i32) -> i32 {
    unsafe {
        if !INITIALIZED {
            initialize();
        }
    }

    nums.sort_unstable();

    let mut ans = 0;
    let mut s = 1;
    for i in 0..nums.len() {
        ans = (ans + s * (nums[i] as i64 + nums[nums.len() - 1 - i] as i64)) % MOD;
        s = (s * 2 - comb(i, k as usize - 1) + MOD) % MOD;
    }

    ans as i32
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut stdin_lock = stdin.lock();
    let mut buffer = String::new();

    // Read n and k
    stdin_lock.read_line(&mut buffer)?;
    let mut iter = buffer.split_whitespace();
    let n: usize = iter.next().unwrap().parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Failed to parse n"))?;
    let k: i32 = iter.next().unwrap().parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Failed to parse k"))?;

    // Allocate and read the array
    let mut nums: Vec<i32> = Vec::with_capacity(n);
    for _ in 0..n {
        buffer.clear();
        stdin_lock.read_line(&mut buffer)?;
        let num: i32 = buffer.trim().parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Failed to parse nums"))?;
        nums.push(num);
    }

    // Calculate and print the result
    let result = min_max_sums(&mut nums, k);
    writeln!(stdout, "{}", result)?;

    Ok(())
}