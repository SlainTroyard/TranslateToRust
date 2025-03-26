use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn count_winning_sequences(s: String) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let mut mp = [0; 128];
        mp[b'F' as usize] = 0;
        mp[b'W' as usize] = 1;
        mp[b'E' as usize] = 2;

        let n = s.len();
        // Create a 3D array for dynamic programming
        // f[i][j][ban] represents the count of valid sequences
        let mut f = vec![vec![[0, 0, 0]; n * 2 + 1]; n + 1];
        
        // Initialize base cases
        for j in (n + 1)..=(n * 2) {
            f[0][j] = [1, 1, 1];
        }

        let mut pow2 = 1;
        for i in 0..n {
            let x = mp[s.as_bytes()[i] as usize];
            pow2 = (pow2 * 2) % MOD;
            
            for j in -i as i32..(n as i32 - i as i32) {
                for ban in 0..3 {
                    if j > (i + 1) as i32 {
                        f[i + 1][(j + n as i32) as usize][ban] = pow2;
                        continue;
                    }
                    
                    // Calculate result for this state
                    let mut res = 0;
                    for k in 0..3 {
                        if i == n - 1 || k != ban {
                            let mut score = (k as i32 - x as i32 + 3) % 3;
                            if score == 2 {
                                score = -1;
                            }
                            res = (res + f[i][(j + score + n as i32) as usize][k]) % MOD;
                        }
                    }
                    f[i + 1][(j + n as i32) as usize][ban] = res;
                }
            }
        }
        
        f[n][n][0]
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read count
    let count: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read string
    let s = lines.next().unwrap().unwrap();
    
    let solution = Solution;
    println!("{}", solution.count_winning_sequences(s));
}