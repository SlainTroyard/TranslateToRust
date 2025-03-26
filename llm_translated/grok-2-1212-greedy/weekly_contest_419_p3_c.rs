use std::io::{self, Read};

const MAX_SIZE: i64 = 1e9 as i64 + 7;

fn count_winning_sequences(s: &str) -> i64 {
    let mut dp = [[[0; 2001]; 3]; 2];
    let res = [
        [0, -1, 1],
        [1, 0, -1],
        [-1, 1, 0],
    ];
    let c_arr = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    let s_len = s.len();
    let mut ans = 0;

    // Initialize c_arr
    let mut c_arr = c_arr;
    c_arr['F' as usize - 'A' as usize] = 0;
    c_arr['W' as usize - 'A' as usize] = 1;
    c_arr['E' as usize - 'A' as usize] = 2;

    // Initialize dp for the first character
    for i in 0..=2 {
        let c = c_arr[s.chars().nth(0).unwrap() as usize - 'A' as usize];
        let score = res[i][c];
        dp[0][i][1000 + score] = 1;
    }

    // Process the rest of the string
    for i in 1..s_len {
        dp[i % 2] = [[0; 2001]; 3];
        for j in 0..=2 {
            let c = c_arr[s.chars().nth(i).unwrap() as usize - 'A' as usize];
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

    // Calculate the final answer
    for i in 0..=2 {
        for j in 1001..=2000 {
            ans += dp[(s_len - 1) % 2][i][j];
            ans %= MAX_SIZE;
        }
    }

    ans
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();
    let char_size: usize = lines.next().unwrap().parse().unwrap();
    let s = lines.next().unwrap().to_string();

    let result = count_winning_sequences(&s);
    println!("{}", result);

    Ok(())
}