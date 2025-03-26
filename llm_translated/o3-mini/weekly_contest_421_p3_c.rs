use std::error::Error;
use std::io::{self, BufRead};

// Compute the greatest common divisor (GCD) using Euclid's algorithm.
fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = a % b;
        a = b;
        b = temp;
    }
    a
}

// This function implements the solution described in the original C code.
// It computes the number of valid subsequence pairs based on the input array.
fn subsequence_pair_count(nums: &[i32]) -> i32 {
    const MOD: i64 = 1_000_000_007;
    const MX: usize = 201; // We use indices 1..200 (index 0 unused)

    // Precompute least common multiples (lcms) for all pairs [1, MX-1].
    let mut lcms = vec![vec![0; MX]; MX];
    for i in 1..MX {
        for j in 1..MX {
            let g = gcd(i as i32, j as i32);
            lcms[i][j] = (i as i32 * j as i32 / g) as i32;
        }
    }

    // Precompute powers of 2 and 3 modulo MOD for a range [0, MX-1].
    let mut pow2 = vec![0i64; MX];
    let mut pow3 = vec![0i64; MX];
    pow2[0] = 1;
    pow3[0] = 1;
    for i in 1..MX {
        pow2[i] = (pow2[i - 1] * 2) % MOD;
        pow3[i] = (pow3[i - 1] * 3) % MOD;
    }

    // Precompute the Möbius function, mu, for numbers 1..MX-1.
    let mut mu = vec![0i32; MX];
    mu[1] = 1;
    for i in 1..MX {
        // j runs over multiples of i greater than i
        for j in ((2 * i)..MX).step_by(i) {
            mu[j] -= mu[i];
        }
    }

    // Find the maximum value in nums.
    let m = nums.iter().copied().max().unwrap_or(0) as usize;

    // Count occurrences for each number up to m.
    // cnt[v] indicates how many numbers in nums are divisible by v.
    let mut cnt = vec![0; m + 1];
    for &num in nums.iter() {
        // Since numbers in nums should be at least 1, we do not check for zero.
        // If num is greater than m, it will be skipped.
        if let Some(elem) = cnt.get_mut(num as usize) {
            *elem += 1;
        }
    }
    // Propagate counts to include multiples.
    for i in 1..=m {
        let mut j = i * 2;
        while j <= m {
            cnt[i] += cnt[j];
            j += i;
        }
    }

    // Allocate a 2D vector f with dimensions (m+1) x (m+1).
    // f[g1][g2] will be computed according to the original formula.
    let mut f = vec![vec![0i64; m + 1]; m + 1];
    for g1 in 1..=m {
        for g2 in 1..=m {
            // Lookup lcms using our precomputed array.
            // Since our lcms array is computed for indices 1..MX-1 and we assume m ≤ MX-1,
            // we can safely index lcms[g1][g2].
            let l = lcms[g1][g2];
            // Only use the count if l ≤ m, otherwise use 0.
            let c = if (l as usize) <= m { cnt[l as usize] } else { 0 };
            let c1 = cnt[g1];
            let c2 = cnt[g2];
            // Compute the terms modulo MOD.
            let term1 = (pow3[c] * pow2[c1 + c2 - 2 * c]) % MOD;
            // Adjusting the term to avoid negative numbers before taking modulus.
            let term2 = (term1 - pow2[c1] - pow2[c2] + 1) % MOD;
            f[g1][g2] = (term2 + MOD) % MOD;
        }
    }

    // Inclusion-Exclusion: sum over all pairs scaled by Möbius function values.
    let mut ans: i64 = 0;
    for i in 1..=m {
        let max_jk = m / i;
        for j in 1..=max_jk {
            for k in 1..=max_jk {
                let gj = j * i;
                let gk = k * i;
                // Use i64 for arithmetic and cast mu from i32 to i64.
                ans = (ans + (mu[j] as i64 * mu[k] as i64 % MOD) * f[gj][gk]) % MOD;
            }
        }
    }
    ans = (ans % MOD + MOD) % MOD;
    ans as i32
}

fn main() -> Result<(), Box<dyn Error>> {
    // Create a buffered reader for standard input.
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first non-empty line for the integer n.
    let first_line = loop {
        match lines.next() {
            Some(line) => {
                let line = line?;
                if !line.trim().is_empty() {
                    break line;
                }
            }
            None => return Err("No input provided".into()),
        }
    };

    // The first value in the first line is the number of elements.
    let n: usize = first_line
        .split_whitespace()
        .next()
        .ok_or("Failed to parse n")?
        .parse()?;

    // Collect n numbers; they might be on the same line or spread across multiple lines.
    let mut nums = Vec::with_capacity(n);
    while nums.len() < n {
        let line = match lines.next() {
            Some(line) => line?,
            None => break,
        };
        for token in line.split_whitespace() {
            if nums.len() < n {
                nums.push(token.parse::<i32>()?);
            } else {
                break;
            }
        }
    }

    if nums.len() < n {
        return Err("Not enough numbers provided".into());
    }

    // Compute the result using the derived subsequence_pair_count function.
    let result = subsequence_pair_count(&nums);

    // Print the result to stdout; matches the exact output format expected.
    println!("{}", result);

    Ok(())
}