use std::io::{self, BufRead, Write};

const MOD: i64 = 1_000_000_007;

struct Solution;

impl Solution {
    fn count_winning_sequences(s: &str) -> i64 {
        let mp = ['F', 'W', 'E'].iter().enumerate().map(|(i, &c)| (c, i)).collect::<std::collections::HashMap<_, _>>();
        let n = s.len();
        let mut f = vec![vec![[0; 3]; n * 2 + 1]; n + 1];
        for j in n + 1..=n * 2 {
            f[0][j] = [1, 1, 1];
        }

        let mut pow2 = 1;
        for i in 0..n {
            let x = mp[&s.chars().nth(i).unwrap()];
            pow2 = pow2 * 2 % MOD;
            for j in -i..n - i {
                for ban in 0..3 {
                    if j > i + 1 {
                        f[i + 1][j + n][ban] = pow2;
                        continue;
                    }
                    let res = &mut f[i + 1][j + n][ban];
                    for k in 0..3 {
                        if i == n - 1 || k != ban {
                            let score = (k - x + 3) % 3;
                            let score = if score == 2 { -1 } else { score as isize };
                            *res = (*res + f[i][j + score + n][k]) % MOD;
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
    let mut stdout = io::stdout();
    let mut stdin_lock = stdin.lock();
    let mut buffer = String::new();

    // Read the number of test cases
    stdin_lock.read_line(&mut buffer).expect("Failed to read line");
    let count: usize = buffer.trim().parse().expect("Failed to parse count");

    // Read the string
    buffer.clear();
    stdin_lock.read_line(&mut buffer).expect("Failed to read line");
    let s = buffer.trim();

    let solution = Solution;
    let result = solution.count_winning_sequences(s);

    writeln!(stdout, "{}", result).expect("Failed to write result");
}