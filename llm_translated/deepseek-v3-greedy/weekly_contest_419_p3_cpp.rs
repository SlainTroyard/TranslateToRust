use std::collections::HashMap;
use std::io;

const MOD: i32 = 1_000_000_007;

fn count_winning_sequences(s: String) -> i32 {
    let mut mp = HashMap::new();
    mp.insert('F', 0);
    mp.insert('W', 1);
    mp.insert('E', 2);

    let n = s.len();
    let mut f = vec![vec![[0; 3]; n * 2 + 1]; n + 1];

    for j in n + 1..=n * 2 {
        f[0][j] = [1, 1, 1];
    }

    let mut pow2 = 1;
    for i in 0..n {
        let x = mp[&s.chars().nth(i).unwrap()];
        pow2 = pow2 * 2 % MOD;
        for j in -i as i32..(n - i) as i32 {
            for ban in 0..3 {
                if j > (i + 1) as i32 {
                    f[i + 1][(j + n as i32) as usize][ban] = pow2;
                    continue;
                }
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

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let count: i32 = input.trim().parse().expect("Invalid input");

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let s = input.trim().to_string();

    let result = count_winning_sequences(s);
    println!("{}", result);
}