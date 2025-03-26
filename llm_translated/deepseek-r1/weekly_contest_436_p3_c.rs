use std::io::{self, Read};

fn count_substrings(s: &str) -> i64 {
    let mut ans = 0;
    let mut f = [[0i64; 9]; 10]; // [modulus 1-9][remainder 0-8]

    for c in s.chars() {
        let d = c.to_digit(10).expect("invalid digit") as i32;
        let d_mod = d as usize;

        for m in 1..10 { // Process modulus from 1 to 9
            let m_i32 = m as i32;
            let mut nf = [0i64; 9];

            // Single digit substring case
            let rem_single = (d % m_i32) as usize;
            nf[rem_single] = 1;

            // Extend previous substrings with current digit
            for rem_prev in 0..m_i32 {
                let rem_prev = rem_prev as usize;
                let count = f[m][rem_prev];
                let new_rem = ((rem_prev as i32 * 10 + d) % m_i32) as usize;
                nf[new_rem] += count;
            }

            // Update current modulus state
            for rem in 0..m_i32 {
                let rem = rem as usize;
                f[m][rem] = nf[rem];
            }
        }

        // Accumulate valid substrings ending with current digit
        ans += f[d_mod][0];
    }

    ans
}

fn main() {
    let mut input = String::new();
    if let Err(_) = io::stdin().read_line(&mut input) {
        eprintln!("Error reading input");
        std::process::exit(1);
    }
    let s = input.trim();

    println!("{}", count_substrings(s));
}