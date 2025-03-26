use std::io::{self, BufRead, Write};

const MOD: i64 = 1_000_000_007;
const MX: usize = 201; // Maximum value constant

// Function to compute the greatest common divisor (Euclid's algorithm)
fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn main() -> io::Result<()> {
    // Prepare fast input and output handling.
    let stdin = io::stdin();
    let mut reader = stdin.lock().lines();
    let stdout = io::stdout();
    let mut writer = io::BufWriter::new(stdout.lock());
    
    // Precompute lcm for numbers in [1, MX) into a 2D array.
    // We use lcm(a, b) = a / gcd(a, b) * b.
    let mut lcms = [[0usize; MX]; MX];
    for i in 1..MX {
        for j in 1..MX {
            let g = gcd(i, j);
            lcms[i][j] = i / g * j;
        }
    }
    
    // Precompute power arrays (for 2 and 3) modulo MOD.
    let mut pow2 = [0i64; MX];
    let mut pow3 = [0i64; MX];
    pow2[0] = 1;
    pow3[0] = 1;
    for i in 1..MX {
        pow2[i] = (pow2[i - 1] * 2) % MOD;
        pow3[i] = (pow3[i - 1] * 3) % MOD;
    }
    
    // Precompute the Möbius function (mu) for values in [1, MX)
    let mut mu = [0i32; MX];
    mu[1] = 1;
    for i in 1..MX {
        let mut j = i * 2;
        while j < MX {
            mu[j] -= mu[i];
            j += i;
        }
    }
    
    // READ INPUT
    // The first number is n, then n integers.
    // Input parsing follows exactly the input behaviour of the C++ code.
    let first_line = reader.next().unwrap()?;
    let n: usize = first_line.trim().parse().expect("Invalid input for n");
    
    // Read the next line(s) that include n numbers.
    // We collect all whitespace-separated tokens from the remaining input.
    let mut nums = Vec::with_capacity(n);
    while nums.len() < n {
        let line = reader.next().unwrap()?;
        for token in line.trim().split_whitespace() {
            if nums.len() < n {
                nums.push(token.parse::<usize>().expect("Invalid input for a number"));
            }
        }
    }
    
    // m is the maximum value in nums.
    let m = *nums.iter().max().unwrap();
    
    // Count frequency for each number in [1, m].
    // Also, later we'll accumulate counts for multiples.
    let mut cnt = vec![0; m + 1];
    for &x in &nums {
        if x <= m {
            cnt[x] += 1;
        }
    }
    // For every number i in 1..=m, add counts of its multiples
    for i in 1..=m {
        let mut j = i * 2;
        while j <= m {
            cnt[i] += cnt[j];
            j += i;
        }
    }
    
    // Precompute a 2D table f[g1][g2] for 1 <= g1, g2 <= m.
    // f[g1][g2] = (pow3[cnt[lcm(g1, g2)]] * pow2[cnt[g1] + cnt[g2] - 2*cnt[lcm(g1, g2)]]
    //               - pow2[cnt[g1]] - pow2[cnt[g2]] + 1) mod MOD.
    let mut f = vec![vec![0i64; m + 1]; m + 1];
    for g1 in 1..=m {
        for g2 in 1..=m {
            let l = lcms[g1][g2];
            let c = if l <= m { cnt[l] } else { 0 };
            let c1 = cnt[g1];
            let c2 = cnt[g2];
            // Calculate f[g1][g2] under modulo arithmetic.
            let mut term = pow3[c] * pow2[c1 + c2 - 2 * c] % MOD;
            term = (term - pow2[c1] - pow2[c2] + 1) % MOD;
            if term < 0 {
                term += MOD;
            }
            f[g1][g2] = term;
        }
    }
    
    // Calculate the final answer using triple nested loops with Möbius inversion.
    let mut ans: i64 = 0;
    for i in 1..=m {
        let max_jk = m / i;
        for j in 1..=max_jk {
            for k in 1..=max_jk {
                // Multiply each contribution and add to answer.
                ans = (ans + (mu[j] as i64) * (mu[k] as i64) % MOD * f[j * i][k * i] % MOD) % MOD;
            }
        }
    }
    ans = (ans % MOD + MOD) % MOD; // Ensure answer is non-negative.
    
    // Output the answer.
    writeln!(writer, "{}", ans)?;
    writer.flush()?;
    Ok(())
}