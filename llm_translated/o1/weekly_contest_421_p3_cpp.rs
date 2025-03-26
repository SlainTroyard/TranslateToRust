// Translated from the original C++ solution for LeetCode Weekly Contest 421 Problem 3.
// This Rust program reads an integer n, followed by n integers in the same or subsequent line(s),
// and outputs the result of the subsequencePairCount function. The logic is preserved exactly.

// We use once_cell::sync::Lazy for one-time global initialization.
use once_cell::sync::Lazy;
use std::io;
use std::io::BufRead;

// Constants
const MOD: i64 = 1_000_000_007;
const MX: usize = 201;

// Compute GCD and LCM for small integer ranges
fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

// Precompute LCMs for all pairs [1..200]
static LCM: Lazy<[[usize; MX]; MX]> = Lazy::new(|| {
    let mut lcms = [[0; MX]; MX];
    for i in 1..MX {
        for j in 1..MX {
            lcms[i][j] = lcm(i, j);
        }
    }
    lcms
});

// Precompute powers of 2 up to 200
static POW2: Lazy<[i64; MX]> = Lazy::new(|| {
    let mut arr = [0; MX];
    arr[0] = 1;
    for i in 1..MX {
        arr[i] = (arr[i - 1] * 2) % MOD;
    }
    arr
});

// Precompute powers of 3 up to 200
static POW3: Lazy<[i64; MX]> = Lazy::new(|| {
    let mut arr = [0; MX];
    arr[0] = 1;
    for i in 1..MX {
        arr[i] = (arr[i - 1] * 3) % MOD;
    }
    arr
});

// Precompute Möbius function mu for [1..200]
// The logic is the same as the original: mu[1] = 1, then repeatedly subtract mu[i] from mu[j].
static MU: Lazy<[i64; MX]> = Lazy::new(|| {
    let mut mu = [0; MX];
    mu[1] = 1;
    for i in 1..MX {
        let mut j = i * 2;
        while j < MX {
            mu[j] -= mu[i];
            j += i;
        }
    }
    mu
});

// A struct to mirror the C++ "Solution" class.
struct Solution;

impl Solution {
    fn subsequence_pair_count(&self, nums: &[i32]) -> i64 {
        // 1) Find the maximum element in nums
        let m = match nums.iter().max() {
            Some(&val) => val,
            None => 0,
        } as usize;

        // 2) Build frequency array cnt, where cnt[x] = number of occurrences of x in nums
        let mut cnt = vec![0i32; m + 1];
        for &x in nums {
            cnt[x as usize] += 1;
        }

        // 3) Accumulate counts: for each i, add to cnt[i] all cnt[j] where j is multiple of i
        //    This means cnt[i] will be the count of numbers in nums that are multiples of i.
        for i in 1..=m {
            let mut j = i * 2;
            while j <= m {
                cnt[i] += cnt[j];
                j += i;
            }
        }

        // 4) Prepare a 2D array f of size (m+1)×(m+1)
        //    f[g1][g2] = (pow3[c] * pow2[c1 + c2 - 2c] - pow2[c1] - pow2[c2] + 1) mod
        //    where c = cnt[lcm(g1,g2)], c1 = cnt[g1], c2 = cnt[g2]
        let mut f = vec![vec![0i64; m + 1]; m + 1];
        for g1 in 1..=m {
            for g2 in 1..=m {
                let l = LCM[g1][g2];
                let c = if l <= m { cnt[l] } else { 0 };
                let c1 = cnt[g1];
                let c2 = cnt[g2];

                // We do everything in i64, applying modulus carefully
                let c64 = c as i64;
                let c1_64 = c1 as i64;
                let c2_64 = c2 as i64;

                // pow3[c], pow2[c1 + c2 - 2c]
                let tmp1 = (POW3[c64 as usize] * POW2[(c1_64 + c2_64 - 2 * c64) as usize]) % MOD;

                // (tmp1 - pow2[c1] - pow2[c2] + 1) mod
                let mut val = tmp1;
                val = (val - POW2[c1_64 as usize]) % MOD;
                val = (val - POW2[c2_64 as usize]) % MOD;
                val = (val + 1) % MOD;

                // Normalize to [0..MOD-1]
                if val < 0 {
                    val += MOD;
                }
                f[g1][g2] = val;
            }
        }

        // 5) Compute the final answer:
        //    ans = sum over i=1..m { sum over j=1..(m/i) { sum over k=1..(m/i) { mu[j]*mu[k]*f[j*i][k*i] } } } mod
        let mut ans: i64 = 0;
        for i in 1..=m {
            let limit = m / i;
            for j in 1..=limit {
                for k in 1..=limit {
                    let tmp = MU[j] * MU[k] * f[j * i][k * i];
                    ans = (ans + tmp) % MOD;
                }
            }
        }

        // Normalize final answer
        if ans < 0 {
            ans += MOD;
        }
        ans
    }
}

fn main() {
    // Read integer n
    let mut input_line = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line for n");
    let n: usize = input_line.trim().parse().expect("Failed to parse n");

    // Read the n integers into nums
    let mut nums: Vec<i32> = Vec::with_capacity(n);
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // We gather integers from however many lines it takes until we have n of them
    while nums.len() < n {
        let line = match lines.next() {
            Some(Ok(l)) => l,
            Some(Err(e)) => panic!("Error reading line: {}", e),
            None => break, // If input ends unexpectedly
        };
        for token in line.split_whitespace() {
            if nums.len() == n {
                break;
            }
            nums.push(token.parse().expect("Failed to parse integer"));
        }
    }

    // Solve and print the result
    let solution = Solution {};
    let answer = solution.subsequence_pair_count(&nums);
    println!("{}", answer);
}