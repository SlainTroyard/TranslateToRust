use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();
    let count: i32 = lines.next().unwrap().parse().unwrap();
    let s: String = lines.next().unwrap().to_string();

    let solution = Solution {};
    println!("{}", solution.count_winning_sequences(s));
}

struct Solution {}

impl Solution {
    pub fn count_winning_sequences(s: String) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let mut mp: [i32; 128] = [0; 128];
        mp['F' as usize] = 0;
        mp['W' as usize] = 1;
        mp['E' as usize] = 2;

        let n = s.len();
        let mut f: Vec<Vec<[i32; 3]>> = vec![vec![[0; 3]; n * 2 + 1]; n + 1];

        for j in n + 1..=n * 2 {
            f[0][j][0] = 1;
            f[0][j][1] = 1;
            f[0][j][2] = 1;
        }

        let mut pow2 = 1;
        for i in 0..n {
            let x = mp[s.as_bytes()[i] as usize];
            pow2 = pow2 * 2 % MOD;
            for j in -i..n - i {
                for ban in 0..3 {
                    if j > i as i32 + 1 {
                        f[i + 1][(j + n as i32) as usize][ban as usize] = pow2;
                        continue;
                    }
                    let res = &mut f[i + 1][(j + n as i32) as usize][ban as usize]; // 注意这里是引用
                    for k in 0..3 {
                        if i == n - 1 || k != ban {
                            let mut score = (k - x + 3) % 3;
                            if score == 2 {
                                score = -1;
                            }
                            *res = (*res + f[i][(j + score + n as i32) as usize][k as usize]) % MOD;
                        }
                    }
                }
            }
        }
        f[n][n][0]
    }
}