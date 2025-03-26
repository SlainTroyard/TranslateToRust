use std::io::{self, BufRead};
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn count_winning_sequences(s: &str) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let mut mp = HashMap::new();
        mp.insert('F', 0);
        mp.insert('W', 1);
        mp.insert('E', 2);

        let n = s.len();
        let mut f = vec![vec![[0; 3]; n * 2 + 1]; n + 1];
        for j in (n + 1)..=(n * 2) {
            f[0][j] = [1, 1, 1];
        }

        let mut pow2 = 1;
        for i in 0..n {
            let x = mp[&s.chars().nth(i).unwrap()];
            pow2 = pow2 * 2 % MOD;
            for j in (-i..(n - i)) {
                for ban in 0..3 {
                    if j > i + 1 {
                        f[i + 1][(j + n) as usize][ban] = pow2;
                        continue;
                    }
                    let res = &mut f[i + 1][(j + n) as usize][ban];
                    for k in 0..3 {
                        if i == n - 1 || k != ban {
                            let mut score = (k - x + 3) % 3;
                            if score == 2 {
                                score = -1;
                            }
                            *res = (*res + f[i][(j + score + n) as usize][k]) % MOD;
                        }
                    }
                }
            }
        }
        f[n][n][0]
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the count (not used in the solution, but required for input format)
    let count: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read the string `s`
    let s = lines.next().unwrap().unwrap();

    let solution = Solution;
    let result = solution.count_winning_sequences(&s);
    println!("{}", result);
}