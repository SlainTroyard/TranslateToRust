use std::io::{self, BufRead};

const MAX_SIZE: i64 = 1e9 as i64 + 7;

fn count_winning_sequences(s: &str) -> i64 {
    let mut dp = [[[0; 2001]; 3]; 2];
    let res = [
        [0, -1, 1],
        [1, 0, -1],
        [-1, 1, 0],
    ];
    let c_arr = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let s_len = s.len();
    let mut ans = 0;

    let c_arr = [
        c_arr[0], c_arr[1], c_arr[2], c_arr[3], c_arr[4], c_arr[5],
        c_arr[6], c_arr[7], c_arr[8], c_arr[9], c_arr[10], c_arr[11],
        c_arr[12], c_arr[13], c_arr[14], c_arr[15], c_arr[16], c_arr[17],
        c_arr[18], c_arr[19], c_arr[20], c_arr[21], c_arr[22], c_arr[23],
        0,  // 'F' - 'A' = 5
        0,  // 'G' - 'A' = 6
        0,  // 'H' - 'A' = 7
        0,  // 'I' - 'A' = 8
        0,  // 'J' - 'A' = 9
        0,  // 'K' - 'A' = 10
        0,  // 'L' - 'A' = 11
        0,  // 'M' - 'A' = 12
        0,  // 'N' - 'A' = 13
        0,  // 'O' - 'A' = 14
        0,  // 'P' - 'A' = 15
        0,  // 'Q' - 'A' = 16
        0,  // 'R' - 'A' = 17
        0,  // 'S' - 'A' = 18
        0,  // 'T' - 'A' = 19
        0,  // 'U' - 'A' = 20
        0,  // 'V' - 'A' = 21
        1,  // 'W' - 'A' = 22
        0,  // 'X' - 'A' = 23
        0,  // 'Y' - 'A' = 24
        2,  // 'Z' - 'A' = 25
    ];

    for i in 0..=2 {
        let c = c_arr[s.as_bytes()[0] as usize - b'A' as usize];
        let score = res[i][c];
        dp[0][i][1000 + score as usize] = 1;
    }

    for i in 1..s_len {
        dp[i % 2] = [[0; 2001]; 3];
        for j in 0..=2 {
            let c = c_arr[s.as_bytes()[i] as usize - b'A' as usize];
            let score = res[j][c];
            for k in 0..=2000 {
                for j1 in 0..=2 {
                    if j1 != j {
                        if k as i32 - score >= 0 && k as i32 - score <= 2000 {
                            dp[i % 2][j][k] += dp[(i - 1) % 2][j1][(k as i32 - score) as usize];
                            dp[i % 2][j][k] %= MAX_SIZE;
                        }
                    }
                }
            }
        }
    }

    for i in 0..=2 {
        for j in 1001..=2000 {
            ans += dp[(s_len - 1) % 2][i][j];
            ans %= MAX_SIZE;
        }
    }

    ans
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let char_size: usize = lines.next().unwrap()?.trim().parse().unwrap();
    let s: String = lines.next().unwrap()?.trim().chars().take(char_size).collect();

    let result = count_winning_sequences(&s);
    println!("{}", result);

    Ok(())
}