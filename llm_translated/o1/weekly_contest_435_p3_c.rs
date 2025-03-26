use std::io::{self, BufRead};
use std::process;

// Calculate greatest common divisor
fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

// Our main DP-based function, equivalent to minimumIncrements in the C code
fn minimum_increments(nums: &[i64], target: &[i64]) -> i64 {
    let n = nums.len();
    let m = target.len();

    // Precompute g array for divisibility checks
    // g[i] represents the least common multiple of the selected targets,
    // determined by the bits set in i.
    let size = 1 << m;
    let mut g = vec![1_i64; size];
    for i in 0..size {
        let mut val = 1_i64;
        for j in 0..m {
            if (i >> j) & 1 == 1 {
                // val = val / gcd(val, target[j]) * target[j]
                let common = gcd(val, target[j]);
                val = val / common * target[j];
            }
        }
        g[i] = val;
    }

    // We'll use a large INF to represent infinity in our DP
    const INF: i64 = 1_000_000_000_000_000_000; // 1e18

    // f will be our DP array: we alternate between f[0] and f[1] for memory
    // equivalence of the C code's 2D array f[2][1 << m].
    let mut f = [vec![INF; size], vec![INF; size]];

    // Initialize DP
    f[0][0] = 0;

    // Perform the DP transitions
    for i in 1..=n {
        let curr = i & 1;
        let prev = curr ^ 1;

        // Copy over previous states (like f[i][j] = f[i-1][j])
        for j in 0..size {
            f[curr][j] = f[prev][j];
        }

        // Enumerate submasks of j
        for j in 0..size {
            let mut k = j;
            while k > 0 {
                let v = {
                    // v = ((nums[i-1] + g[k] - 1) / g[k]) * g[k] - nums[i-1]
                    let numerator = nums[i - 1] + g[k] - 1;
                    (numerator / g[k]) * g[k] - nums[i - 1]
                };
                let alt = f[prev][j ^ k].saturating_add(v);
                if alt < f[curr][j] {
                    f[curr][j] = alt;
                }
                k = (k - 1) & j; // move to the next submask
            }
        }
    }

    // Final answer is f[n & 1][(1 << m) - 1]
    f[n & 1][size - 1]
}

// Helper to read tokens from stdin and store them in a vector of strings
fn read_tokens() -> Vec<String> {
    let stdin = io::stdin();
    let mut tokens = Vec::new();
    for line in stdin.lock().lines().flatten() {
        for token in line.split_whitespace() {
            tokens.push(token.to_string());
        }
    }
    tokens
}

fn main() {
    // Read all tokens from stdin
    let tokens = read_tokens();
    let mut idx = 0;

    // Helper macro to parse the next token as i64, handling errors
    macro_rules! next_i64 {
        () => {{
            if idx >= tokens.len() {
                eprintln!("Error: insufficient input data");
                process::exit(1);
            }
            match tokens[idx].parse::<i64>() {
                Ok(val) => {
                    idx += 1;
                    val
                }
                Err(_) => {
                    eprintln!("Error reading input for integer");
                    process::exit(1);
                }
            }
        }};
    }

    // Read n and m
    let n = {
        let tmp = next_i64!();
        if tmp < 0 || tmp > i64::from(i32::MAX) {
            eprintln!("Error: invalid n");
            process::exit(1);
        }
        tmp as usize
    };

    let m = {
        let tmp = next_i64!();
        if tmp < 0 || tmp > i64::from(i32::MAX) {
            eprintln!("Error: invalid m");
            process::exit(1);
        }
        tmp as usize
    };

    // Read nums array
    let mut nums = Vec::with_capacity(n);
    for i in 0..n {
        let val = next_i64!();
        if val < i64::from(i32::MIN) || val > i64::from(i32::MAX) {
            eprintln!("Error reading input for nums[{}]", i);
            process::exit(1);
        }
        nums.push(val);
    }

    // Read target array
    let mut target = Vec::with_capacity(m);
    for i in 0..m {
        let val = next_i64!();
        if val < i64::from(i32::MIN) || val > i64::from(i32::MAX) {
            eprintln!("Error reading input for target[{}]", i);
            process::exit(1);
        }
        target.push(val);
    }

    // Compute result (same as C function call)
    let result = minimum_increments(&nums, &target);

    // Print result as an integer (matching printf("%d\n", result) in the C code)
    println!("{}", result as i32);
}