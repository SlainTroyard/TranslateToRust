use std::io;
use std::io::Read;

fn count_winning_sequences(s: &str) -> i32 {
    const MAX_SIZE: i32 = 1_000_000_007;
    let mut dp = [[[0; 2001]; 3]; 2];
    let mut res = [[0; 3]; 3];
    let mut c_arr = [0; 26];
    let s_len = s.len();
    let mut ans = 0;

    c_arr[('F' as u8 - 'A' as u8) as usize] = 0;
    c_arr[('W' as u8 - 'A' as u8) as usize] = 1;
    c_arr[('E' as u8 - 'A' as u8) as usize] = 2;

    res[0][2] = 1;
    res[0][1] = -1;
    res[2][0] = -1;
    res[1][0] = 1;
    res[2][1] = 1;
    res[1][2] = -1;

    for i in 0..=2 {
        let c = c_arr[(s.as_bytes()[0] - b'A') as usize];
        let score = res[i][c as usize];
        dp[0][i][(1000 + score) as usize] = 1;
    }

    for i in 1..s_len {
        dp[i % 2] = [[[0; 2001]; 3]; 1][0]; // Clear dp[i % 2]
        for j in 0..=2 {
            let c = c_arr[(s.as_bytes()[i] - b'A') as usize];
            let score = res[j][c as usize];
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
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut lines = buffer.lines();
    let char_size: usize = lines.next().unwrap().parse().unwrap(); // Expect a single line with an integer
    let s: String = lines.next().unwrap().to_string();

    let result = count_winning_sequences(&s);
    println!("{}", result);

    Ok(())
}