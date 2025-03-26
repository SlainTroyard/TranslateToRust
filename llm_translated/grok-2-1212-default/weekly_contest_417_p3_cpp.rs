use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn count_of_substrings(word: String, k: i32) -> i64 {
        const VOWEL_MASK: u32 = 1065233; // Binary representation of vowels
        let mut ans = 0;
        let mut cnt_vowel1 = [0; 21]; // 'u' - 'a' + 1
        let mut cnt_vowel2 = [0; 21];
        let mut size_vowel1 = 0;
        let mut size_vowel2 = 0;
        let mut cnt_consonant1 = 0;
        let mut cnt_consonant2 = 0;
        let mut left1 = 0;
        let mut left2 = 0;

        for b in word.bytes() {
            let b = (b - b'a') as usize;
            if VOWEL_MASK & (1 << b) != 0 {
                if cnt_vowel1[b] == 0 {
                    size_vowel1 += 1;
                }
                cnt_vowel1[b] += 1;
                if cnt_vowel2[b] == 0 {
                    size_vowel2 += 1;
                }
                cnt_vowel2[b] += 1;
            } else {
                cnt_consonant1 += 1;
                cnt_consonant2 += 1;
            }

            while size_vowel1 == 5 && cnt_consonant1 >= k {
                let out = word.as_bytes()[left1] - b'a';
                if VOWEL_MASK & (1 << out) != 0 {
                    cnt_vowel1[out as usize] -= 1;
                    if cnt_vowel1[out as usize] == 0 {
                        size_vowel1 -= 1;
                    }
                } else {
                    cnt_consonant1 -= 1;
                }
                left1 += 1;
            }

            while size_vowel2 == 5 && cnt_consonant2 > k {
                let out = word.as_bytes()[left2] - b'a';
                if VOWEL_MASK & (1 << out) != 0 {
                    cnt_vowel2[out as usize] -= 1;
                    if cnt_vowel2[out as usize] == 0 {
                        size_vowel2 -= 1;
                    }
                } else {
                    cnt_consonant2 -= 1;
                }
                left2 += 1;
            }

            ans += (left1 - left2) as i64;
        }
        ans
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read input
    let word_size: usize = lines.next().unwrap()?.parse().unwrap();
    let word: String = lines.next().unwrap()?;
    let k: i32 = lines.next().unwrap()?.parse().unwrap();

    // Process and output result
    let ans = Solution::count_of_substrings(word, k);
    println!("{}", ans);

    Ok(())
}