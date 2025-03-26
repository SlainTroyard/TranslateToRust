use std::io;

const MAX_SIZE: i32 = 1000000007;

fn count_winning_sequences(s: &str) -> i32 {
    let res = [
        [0, -1, 1],
        [1, 0, -1],
        [-1, 1, 0],
    ];

    let mut c_arr = [0; 26];
    c_arr['F' as usize - 'A' as usize] = 0;
    c_arr['W' as usize - 'A' as usize] = 1;
    c_arr['E' as usize - 'A' as usize] = 2;

    let s_len = s.len();
    if s_len == 0 {
        return 0;
    }

    let mut dp = vec![vec![vec![0; 2001]; 3]; 2];

    let first_char = s.chars().nth(0).unwrap();
    let c = c_arr[(first_char as u8 - b'A') as usize];
    for j in 0..=2 {
        let score = res[j][c];
        let k = 1000 + score;
        if k >= 0 && k < 2001 {
            dp[0][j][k] = 1;
        }
    }

    for i in 1..s_len {
        let current_layer = i % 2;
        let prev_layer = (i - 1) % 2;

        for j in 0..=2 {
            for k in 0..2001 {
                dp[current_layer][j][k] = 0;
            }
        }

        let current_char = s.chars().nth(i).unwrap();
        let c = c_arr[(current_char as u8 - b'A') as usize];

        for j in 0..=2 {
            let score = res[j][c];
            for k in 0..=2000 {
                for j1 in 0..=2 {
                    if j1 != j {
                        let prev_k = k - score;
                        if prev_k >= 0 && prev_k <= 2000 {
                            dp[current_layer][j][k] += dp[prev_layer][j1][prev_k];
                            dp[current_layer][j][k] %= MAX_SIZE;
                        }
                    }
                }
            }
        }
    }

    let mut ans = 0;
    let last_layer = (s_len - 1) % 2;
    for j in 0..=2 {
        for k in 1001..=2000 {
            ans += dp[last_layer][j][k];
            ans %= MAX_SIZE;
        }
    }

    ans
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let char_size: usize = input.trim().parse().expect("Invalid integer");

    let mut s_input = String::new();
    io::stdin().read_line(&mut s_input).expect("Failed to read string");
    let s = s_input.trim();

    let result = count_winning_sequences(s);
    println!("{}", result);
}