use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn length_after_transformations(s: String, t: i32) -> i32 {
        const MOD: i64 = 1e9 as i64 + 7;
        let mut cnt = [0; 26];
        
        // Count the frequency of each character
        for c in s.chars() {
            cnt[(c as u8 - b'a') as usize] += 1;
        }
        
        // Apply transformations t times
        for _ in 1..=t {
            let mut nxt = [0; 26];
            
            // Shift the counts
            for j in 0..25 {
                nxt[j + 1] = cnt[j];
            }
            
            // Handle the 'z' character
            nxt[0] = (nxt[0] + cnt[25]) % MOD;
            nxt[1] = (nxt[1] + cnt[25]) % MOD;
            
            cnt = nxt;
        }
        
        // Calculate the total length
        let ans: i64 = cnt.iter().map(|&x| x as i64).sum::<i64>() % MOD;
        ans as i32
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read input
    let s = lines.next().unwrap()?;
    let t: i32 = lines.next().unwrap()?.parse().unwrap();

    // Calculate and print result
    let solution = Solution;
    println!("{}", solution.length_after_transformations(s, t));

    Ok(())
}