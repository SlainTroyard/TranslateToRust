use std::io;
use std::io::Read;

const MOD: i64 = 1_000_000_007;

struct Solution {}

impl Solution {
    pub fn length_after_transformations(s: String, t: i32) -> i32 {
        let mut cnt: [i64; 26] = [0; 26];
        for c in s.chars() {
            cnt[(c as u8 - b'a') as usize] += 1;
        }

        for _ in 1..=t {
            let mut nxt: [i64; 26] = [0; 26];
            for j in 0..25 {
                nxt[j + 1] = cnt[j];
            }
            nxt[0] = (nxt[0] + cnt[25]) % MOD;
            nxt[1] = (nxt[1] + cnt[25]) % MOD;

            for j in 0..26 {
                cnt[j] = nxt[j];
            }
        }

        let mut ans: i64 = 0;
        for i in 0..26 {
            ans = (ans + cnt[i]) % MOD;
        }

        ans as i32
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();
    let s = lines.next().unwrap().to_string();
    let t = lines.next().unwrap().parse::<i32>().unwrap();

    let solution = Solution {};
    let result = solution.length_after_transformations(s, t);
    println!("{}", result);

    Ok(())
}