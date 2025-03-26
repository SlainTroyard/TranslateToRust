const MOD: usize = 1_000_000_007;

fn main() {
    use std::io;
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let count: usize = input.trim().parse().unwrap();
    
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s = s.trim().to_string();
    
    let solution = Solution;
    println!("{}", solution.count_winning_sequences(&s));
}

struct Solution;

impl Solution {
    fn count_winning_sequences(&self, s: &str) -> usize {
        let mp = ['F', 'W', 'E'].iter()
            .enumerate()
            .fold([0; 128], |mut arr, (i, c)| {
                arr[*c as usize] = i;
                arr
            });
        
        let n = s.len();
        let mut f = vec![vec![vec![0; 3]; 2 * n + 1]; n + 1];
        
        for j in n+1..=2*n {
            f[0][j] = [1, 1, 1];
        }
        
        let mut pow2 = 1;
        for i in 0..n {
            let x = mp[s.as_bytes()[i] as usize];
            pow2 = (pow2 * 2) % MOD;
            
            for j in -i..(n - i) {
                let j = j + n;
                for ban in 0..3 {
                    if j > i + 1 + n {
                        f[i+1][j][ban] = pow2;
                        continue;
                    }
                    
                    let mut res = 0;
                    for k in 0..3 {
                        if i == n - 1 || k != ban {
                            let score = (k as isize - x as isize + 3) % 3;
                            let score = if score == 2 { -1 } else { score };
                            let score = score as usize;
                            let prev_j = j - score;
                            if prev_j >= 0 && prev_j < 2*n + 1 {
                                res = (res + f[i][prev_j][k]) % MOD;
                            }
                        }
                    }
                    f[i+1][j][ban] = res;
                }
            }
        }
        
        f[n][n][0]
    }
}