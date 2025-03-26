use std::io::{self, BufRead};

struct Solution {
    vowel_mask: u32,
}

impl Solution {
    const VOWEL_MASK: u32 = 0b1000001000001000001000001;

    fn new() -> Self {
        Solution { vowel_mask: Self::VOWEL_MASK }
    }

    fn f(&self, word: &str, k: i32) -> i64 {
        let mut ans: i64 = 0;
        let mut cnt1 = [0; 26];
        let mut size1 = 0; // 元音种类数
        let mut cnt2 = 0;
        let mut left = 0;

        for b in word.bytes() {
            let b = (b - b'a') as usize;
            if self.vowel_mask & (1 << b) != 0 {
                cnt1[b] += 1;
                if cnt1[b] == 1 {
                    size1 += 1;
                }
            } else {
                cnt2 += 1;
            }

            while size1 == 5 && cnt2 >= k {
                let out = (word.as_bytes()[left] - b'a') as usize;
                if self.vowel_mask & (1 << out) != 0 {
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

    fn count_of_substrings(&self, word: String, k: i32) -> i64 {
        self.f(&word, k) - self.f(&word, k + 1)
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let word_size: i32 = lines.next().unwrap()?.trim().parse().unwrap();
    let word: String = lines.next().unwrap()?.trim().to_string();
    let k: i32 = lines.next().unwrap()?.trim().parse().unwrap();

    let solution = Solution::new();
    let result = solution.count_of_substrings(word, k);

    println!("{}", result);

    Ok(())
}