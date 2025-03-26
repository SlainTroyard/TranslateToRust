use std::io;

fn gcd(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let temp = a % b;
        a = b;
        b = temp;
    }
    a
}

fn subsequence_pair_count(nums: &[usize]) -> usize {
    const MOD: usize = 1_000_000_007;
    const MX: usize = 201;

    static mut INITIALIZED: bool = false;
    static mut LCMs: [[usize; MX]; MX] = [[0; MX]; MX];
    static mut POW2: [usize; MX] = [0; MX];
    static mut POW3: [usize; MX] = [0; MX];
    static mut MU: [isize; MX] = [0; MX];

    unsafe {
        if !INITIALIZED {
            // Initialize LCMs
            for i in 1..MX {
                for j in 1..MX {
                    let g = gcd(i, j);
                    LCMs[i][j] = (i * j) / g;
                }
            }

            // Initialize POW2 and POW3
            POW2[0] = 1;
            POW3[0] = 1;
            for i in 1..MX {
                POW2[i] = (POW2[i - 1] * 2) % MOD;
                POW3[i] = (POW3[i - 1] * 3) % MOD;
            }

            // Initialize MU
            MU[1] = 1;
            for i in 1..MX {
                for j in (2 * i..MX).step_by(i) {
                    MU[j] -= MU[i];
                }
            }

            INITIALIZED = true;
        }

        // Find maximum value in nums
        let m = *nums.iter().max().unwrap_or(&0);

        // Count occurrences and their multiples
        let mut cnt = vec![0; m + 1];
        for &num in nums {
            cnt[num] += 1;
        }
        for i in 1..=m {
            for j in (2 * i..=m).step_by(i) {
                cnt[i] += cnt[j];
            }
        }

        // Initialize f array
        let mut f = vec![vec![0; m + 1]; m + 1];

        // Fill f array
        for g1 in 1..=m {
            for g2 in 1..=m {
                let l = LCMs[g1][g2];
                let c = if l <= m { cnt[l] } else { 0 };
                let c1 = cnt[g1];
                let c2 = cnt[g2];
                let term1 = (POW3[c] as isize * POW2[c1 + c2 - 2 * c] as isize) % MOD as isize;
                let term2 = (term1 - POW2[c1] as isize - POW2[c2] as isize + 1) % MOD as isize;
                f[g1][g2] = (term2 + MOD as isize) as usize % MOD;
            }
        }

        // Calculate answer using inclusion-exclusion
        let mut ans: isize = 0;
        for i in 1..=m {
            let max_jk = m / i;
            for j in 1..=max_jk {
                for k in 1..=max_jk {
                    let gj = j * i;
                    let gk = k * i;
                    ans += MU[j] * MU[k] * f[gj][gk] as isize;
                }
            }
        }
        ans = (ans % MOD as isize + MOD as isize) % MOD as isize;

        ans as usize
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Failed to parse n");

    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let num: usize = input.trim().parse().expect("Failed to parse num");
        nums.push(num);
    }

    let result = subsequence_pair_count(&nums);
    println!("{}", result);
}