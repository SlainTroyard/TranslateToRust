use std::io;
use std::io::Read;

struct Solution {}

impl Solution {
    pub fn valid_substring_count(word1: String, word2: String) -> i64 {
        let mut count = vec![0; 26];
        for c in word2.chars() {
            count[(c as u8 - b'a') as usize] += 1;
        }

        let n = word1.len();
        let mut pre_count = vec![vec![0; 26]; n + 1];
        for i in 1..=n {
            pre_count[i] = pre_count[i - 1].clone();
            pre_count[i][(word1.as_bytes()[i - 1] - b'a') as usize] += 1;
        }

        let get = |l: usize, r: usize| -> usize {
            let mut border = l;
            let mut left = l;
            let mut right = r;
            while left < right {
                let m = left + (right - left) / 2;
                let mut f = true;
                for i in 0..26 {
                    if pre_count[m][i] - pre_count[border - 1][i] < count[i] {
                        f = false;
                        break;
                    }
                }
                if f {
                    right = m;
                } else {
                    left = m + 1;
                }
            }
            left
        };

        let mut res: i64 = 0;
        for l in 1..=n {
            let r = get(l, n + 1);
            res += (n - r + 1) as i64;
        }
        res
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let len1: usize = lines.next().unwrap().parse()?;
    let word1: String = lines.next().unwrap().to_string();
    let len2: usize = lines.next().unwrap().parse()?;
    let word2: String = lines.next().unwrap().to_string();

    let s = Solution {};
    println!("{}", s.valid_substring_count(word1, word2));

    Ok(())
}