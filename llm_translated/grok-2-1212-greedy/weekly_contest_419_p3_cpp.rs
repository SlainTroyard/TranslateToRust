use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn count_winning_sequences(s: String) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let mp = ['F', 'W', 'E'];
        let mp_map = |c: char| -> usize {
            mp.iter().position(|&x| x == c).unwrap()
        };

        let n = s.len();
        let mut f = vec![vec![[0; 3]; 2 * n + 1]; n + 1];
        for j in n + 1..=2 * n {
            f[0][j] = [1, 1, 1];
        }

        let mut pow2 = 1;
        for i in 0..n {
            let x = mp_map(s.chars().nth(i).unwrap());
            pow2 = (pow2 * 2) % MOD;
            for j in (-(i as i32))..(n as i32 - i as i32) {
                for ban in 0..3 {
                    if j > i as i32 + 1 {
                        f[i + 1][(j + n as i32) as usize][ban] = pow2;
                        continue;
                    }
                    let res = &mut f[i + 1][(j + n as i32) as usize][ban];
                    for k in 0..3 {
                        if i == n - 1 || k != ban {
                            let mut score = (k as i32 - x as i32 + 3) % 3;
                            if score == 2 {
                                score = -1;
                            }
                            *res = (*res + f[i][(j + score + n as i32) as usize][k]) % MOD;
                        }
                    }
                }
            }
        }
        f[n][n][0]
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let count: i32 = lines.next().unwrap()?.trim().parse().unwrap();
    let s = lines.next().unwrap()?.trim().to_string();

    let solution = Solution;
    let result = solution.count_winning_sequences(s);
    println!("{}", result);

    Ok(())
}