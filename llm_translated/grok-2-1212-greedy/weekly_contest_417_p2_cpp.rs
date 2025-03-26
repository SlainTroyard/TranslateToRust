use std::io::{self, Read};

struct Solution;

impl Solution {
    const VOWEL_MASK: u32 = 1065233;

    fn f(word: &str, k: i32) -> i64 {
        let mut ans = 0;
        let mut cnt1 = [0; 26];
        let mut size1 = 0; // 元音种类数
        let mut cnt2 = 0;
        let mut left = 0;
        for &b in word.as_bytes() {
            let b = (b - b'a') as usize;
            if Solution::VOWEL_MASK & (1 << b) != 0 {
                if cnt1[b] == 0 {
                    size1 += 1;
                }
                cnt1[b] += 1;
            } else {
                cnt2 += 1;
            }
            while size1 == 5 && cnt2 >= k {
                let out = (word.as_bytes()[left] - b'a') as usize;
                if Solution::VOWEL_MASK & (1 << out) != 0 {
                    cnt1[out] -= 1;
                    if cnt1[out] == 0 {
                        size1 -= 1;
                    }
                } else {
                    cnt2 -= 1;
                }
                left += 1;
            }
            ans += left as i64;
        }
        ans
    }

    pub fn count_of_substrings(word: String, k: i32) -> i64 {
        Self::f(&word, k) - Self::f(&word, k + 1)
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();
    let word_size: usize = lines.next().unwrap().parse().unwrap();
    let word: String = lines.next().unwrap().to_string();
    let k: i32 = lines.next().unwrap().parse().unwrap();

    let sol = Solution;
    let result = sol.count_of_substrings(word, k);
    println!("{}", result);

    Ok(())
}