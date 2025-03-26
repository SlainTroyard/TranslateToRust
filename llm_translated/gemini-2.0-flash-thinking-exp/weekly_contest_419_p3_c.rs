// Problem: Weekly Contest 419 Problem 3
use std::io;
use std::io::BufRead;

fn count_winning_sequences(s: &str) -> i32 {
    const MAX_SIZE: i32 = 1_000_000_007;
    let mut dp = [[[0i32; 2001]; 3]; 2] ; // dp[2][3][2001] in C, using array instead of Vec for fixed size
    let mut res = [[0i32; 3]; 3]; // res[3][3] in C, using array instead of Vec for fixed size
    let mut c_arr = [0i32; 26]; // c_arr[26] in C, using array instead of Vec for fixed size
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
        let score = res[i as usize][c as usize];
        dp[0][i as usize][(1000 + score) as usize] = 1;
    }

    for i in 1..s_len {
        // memset(dp[i % 2], 0, sizeof(dp[i % 2]));  -> handled by initialization in Rust, no need for memset explicitly in each iteration because we are using array and overwriting
        dp[i % 2] = [[[0i32; 2001]; 3]; 2][0]; // Resetting the current dp layer

        for j in 0..=2 {
            let c = c_arr[(s.as_bytes()[i] - b'A') as usize];
            let score = res[j as usize][c as usize];
            for k in 0..=2000 {
                for j1 in 0..=2 {
                    if j1 != j {
                        if k as i32 - score >= 0 && k as i32 - score <= 2000 {
                            dp[i % 2][j as usize][k as usize] = (dp[i % 2][j as usize][k as usize] + dp[(i - 1) % 2][j1 as usize][(k as i32 - score) as usize]) % MAX_SIZE;
                        }
                    }
                }
            }
        }
    }

    for i in 0..=2 {
        for j in 1001..=2000 {
            ans = (ans + dp[(s_len - 1) % 2][i as usize][j as usize]) % MAX_SIZE;
        }
    }

    ans
}

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    let char_size_str = iterator.next().unwrap().unwrap();
    let _char_size: i32 = char_size_str.trim().parse().unwrap(); // charSize in C, but unused

    let s = iterator.next().unwrap().unwrap();

    println!("{}", count_winning_sequences(&s));
}