use std::io;

struct Solution;

impl Solution {
    pub fn count_winning_sequences(s: &str) -> i64 {
        const MOD: i64 = 1_000_000_007;
        let n = s.len();
        let mut f = vec![vec![[0; 3]; 2 * n + 1]; n + 1];
        
        // Initialize f[0][j] for j from n+1 to 2n (inclusive)
        for j in (n + 1)..=(2 * n) {
            f[0][j] = [1, 1, 1];
        }
        
        let mut pow2 = 1i64;
        
        for i in 0..n {
            let x = match s.chars().nth(i).unwrap() {
                'F' => 0,
                'W' => 1,
                'E' => 2,
                _ => panic!("Invalid character in input string"),
            };
            
            pow2 = (pow2 * 2) % MOD;
            
            // Iterate j from -i to (n - i - 1) inclusive
            for j in (-(i as i32))..(n as i32 - i as i32) {
                for ban in 0..3 {
                    let j_shifted = (j + n as i32) as usize;
                    
                    if j > (i + 1) as i32 {
                        f[i + 1][j_shifted][ban] = pow2 % MOD;
                        continue;
                    }
                    
                    let mut res = 0;
                    for k in 0..3 {
                        // Skip k == ban unless it's the last step
                        if i != n - 1 && k == ban {
                            continue;
                        }
                        
                        let mut score = ((k as i32 - x) + 3) % 3;
                        if score == 2 {
                            score = -1;
                        }
                        
                        let j_prev = j + score;
                        let j_prev_shifted = (j_prev + n as i32) as usize;
                        
                        // Add the contribution from the previous state
                        res += f[i][j_prev_shifted][k];
                        res %= MOD;
                    }
                    
                    f[i + 1][j_shifted][ban] = res % MOD;
                }
            }
        }
        
        f[n][n][0]
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let _count: i32 = input.trim().parse().expect("Invalid count");
    
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let s = input.trim();
    
    let result = Solution::count_winning_sequences(s);
    println!("{}", result);
}