use std::io;

struct Solution;

impl Solution {
    fn count_winning_sequences(s: String) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let mut mp = [0; 128];
        mp[b'F' as usize] = 0;
        mp[b'W' as usize] = 1;
        mp[b'E' as usize] = 2;

        let n = s.len();
        // Create a 3D DP table: [position][score + offset][ban state]
        let mut f = vec![vec![[0; 3]; n * 2 + 1]; n + 1];
        // Initialize the base cases
        for j in (n + 1)..=(n * 2) {
            f[0][j] = [1, 1, 1];
        }

        let mut pow2 = 1;
        for i in 0..n {
            let x = mp[s.as_bytes()[i] as usize];
            pow2 = (pow2 * 2) % MOD;
            for j in (-i as i32)..(n as i32 - i as i32) {
                let j_idx = (j + n as i32) as usize;
                for ban in 0..3 {
                    if j > (i + 1) as i32 {
                        f[i + 1][j_idx][ban] = pow2;
                        continue;
                    }
                    
                    let mut res = 0;
                    for k in 0..3 {
                        if i == n - 1 || k != ban {
                            let mut score = (k as i32 - x as i32 + 3) % 3;
                            if score == 2 {
                                score = -1;
                            }
                            let prev_idx = (j + score + n as i32) as usize;
                            res = (res + f[i][prev_idx][k]) % MOD;
                        }
                    }
                    f[i + 1][j_idx][ban] = res;
                }
            }
        }
        f[n][n][0]
    }
}

fn main() {
    // Read count (though it's not used in the algorithm)
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read count");
    let _count: i32 = input.trim().parse().expect("Failed to parse count");
    
    // Read the string
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read string");
    let s = input.trim().to_string();
    
    let solution = Solution;
    println!("{}", solution.count_winning_sequences(s));
}