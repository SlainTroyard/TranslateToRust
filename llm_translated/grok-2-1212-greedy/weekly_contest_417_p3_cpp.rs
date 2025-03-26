use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn count_of_substrings(word: String, k: i32) -> i64 {
        const VOWEL_MASK: u32 = 1065233;
        let mut ans = 0;
        let mut cnt_vowel1 = [0; 'u' as usize - 'a' as usize + 1];
        let mut cnt_vowel2 = [0; 'u' as usize - 'a' as usize + 1];
        let mut size_vowel1 = 0;
        let mut size_vowel2 = 0;
        let mut cnt_consonant1 = 0;
        let mut cnt_consonant2 = 0;
        let mut left1 = 0;
        let mut left2 = 0;

        for b in word.bytes() {
            let b = (b - b'a') as usize;
            if VOWEL_MASK & (1 << b) != 0 {
                cnt_vowel1[b] += 1;
                if cnt_vowel1[b] == 1 {
                    size_vowel1 += 1;
                }
                cnt_vowel2[b] += 1;
                if cnt_vowel2[b] == 1 {
                    size_vowel2 += 1;
                }
            } else {
                cnt_consonant1 += 1;
                cnt_consonant2 += 1;
            }

            while size_vowel1 == 5 && cnt_consonant1 >= k {
                let out = (word.as_bytes()[left1] - b'a') as usize;
                if VOWEL_MASK & (1 << out) != 0 {
                    cnt_vowel1[out] -= 1;
                    if cnt_vowel1[out] == 0 {
                        size_vowel1 -= 1;
                    }
                } else {
                    cnt_consonant1 -= 1;
                }
                left1 += 1;
            }

            while size_vowel2 == 5 && cnt_consonant2 > k {
                let out = (word.as_bytes()[left2] - b'a') as usize;
                if VOWEL_MASK & (1 << out) != 0 {
                    cnt_vowel2[out] -= 1;
                    if cnt_vowel2[out] == 0 {
                        size_vowel2 -= 1;
                    }
                } else {
                    cnt_consonant2 -= 1;
                }
                left2 += 1;
            }

            ans += left1 as i64 - left2 as i64;
        }
        ans
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap()?;
    let mut iter = first_line.split_whitespace();
    let word_size: i32 = iter.next().unwrap().parse().unwrap();
    let word: String = iter.next().unwrap().to_string();
    let k: i32 = iter.next().unwrap().parse().unwrap();

    let ans = Solution::count_of_substrings(word, k);
    println!("{}", ans);

    Ok(())
}