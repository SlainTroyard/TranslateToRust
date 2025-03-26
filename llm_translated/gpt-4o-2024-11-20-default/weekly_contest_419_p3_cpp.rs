use std::io::{self, Write};
use std::cmp::max;

// Define constants
const MOD: i32 = 1_000_000_007;

struct Solution;

impl Solution {
    pub fn count_winning_sequences(s: &str) -> i32 {
        let mut mp = [0; 128];
        mp['F' as usize] = 0;
        mp['W' as usize] = 1;
        mp['E' as usize] = 2;

        let n = s.len();
        let mut f = vec![vec![[0; 3]; n * 2 + 1]; n + 1];

        for j in (n + 1)..=(n * 2) {
            f[0][j] = [1, 1, 1];
        }

        let mut pow2 = 1;
        for (i, ch) in s.chars().enumerate() {
            let x = mp[ch as usize];
            pow2 = (pow2 * 2) % MOD;

            for j in -i as i32..=(n as i32 - i as i32 - 1) {
                for ban in 0..3 {
                    if j > (i + 1) as i32 {
                        f[i + 1][(j + n as i32) as usize][ban] = pow2;
                        continue;
                    }
                    let res = &mut f[i + 1][(j + n as i32) as usize][ban];
                    for k in 0..3 {
                        if i == n - 1 || k != ban {
                            let mut score = (k - x + 3) % 3;
                            if score == 2 {
                                score = -1;
                            }
                            *res = (*res + f[i][(j + score as i32 + n as i32) as usize][k]) % MOD;
                        }
                    }
                }
            }
        }

        f[n][n][0]
    }
}

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();
    
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let s = input.trim();

    // Solve the problem and output result
    let solution = Solution;
    let result = solution.count_winning_sequences(s);
    println!("{}", result);
}