use std::io;

struct Solution;

impl Solution {
    fn count_winning_sequences(&self, s: &str) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let n = s.len();
        let mut mp = [0; 128];
        mp['F' as usize] = 0;
        mp['W' as usize] = 1;
        mp['E' as usize] = 2;

        let mut f = vec![vec![[0; 3]; 2 * n + 1]; n + 1];
        for j in (n + 1)..=(2 * n) {
            f[0][j] = [1, 1, 1];
        }

        let mut pow2 = 1;
        for i in 0..n {
            let c = s.chars().nth(i).unwrap();
            let x = mp[c as usize];
            pow2 = (pow2 * 2) % MOD;

            for j_val in (-i as i32)..((n - i) as i32) {
                let index_j = (j_val + n as i32) as usize;
                for ban in 0..3 {
                    if j_val > (i as i32 + 1) {
                        f[i + 1][index_j][ban] = pow2;
                    } else {
                        let res = &mut f[i + 1][index_j][ban];
                        *res = 0;
                        for k in 0..3 {
                            if i == n - 1 || k != ban {
                                let mut score = (k as i32 - x + 3) % 3;
                                if score == 2 {
                                    score = -1;
                                }
                                let prev_j_index = (j_val + score + n as i32) as usize;
                                let prev_val = f[i][prev_j_index][k as usize];
                                *res = (*res + prev_val) % MOD;
                            }
                        }
                    }
                }
            }
        }
        f[n][n][0]
    }
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read count");
    let _count: i32 = input.trim().parse().expect("Invalid count");
    
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("Failed to read string");
    let s = s.trim().to_string();
    
    let solution = Solution;
    println!("{}", solution.count_winning_sequences(&s));
}