use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn count_of_substrings(word: &str, k: usize) -> i64 {
        const VOWEL_MASK: u32 = 1065233;
        let mut ans: i64 = 0;
        let mut cnt_vowel1 = [0; 'u' as usize - 'a' as usize + 1];
        let mut cnt_vowel2 = [0; 'u' as usize - 'a' as usize + 1];
        let mut size_vowel1 = 0;
        let mut size_vowel2 = 0; // Number of vowel types
        let mut cnt_consonant1 = 0;
        let mut cnt_consonant2 = 0;
        let mut left1 = 0;
        let mut left2 = 0;

        for (i, b) in word.bytes().enumerate() {
            let b = (b - b'a') as usize;
            if (VOWEL_MASK >> b) & 1 == 1 {
                if cnt_vowel1[b] == 0 {
                    size_vowel1 += 1;
                }
                if cnt_vowel2[b] == 0 {
                    size_vowel2 += 1;
                }
                cnt_vowel1[b] += 1;
                cnt_vowel2[b] += 1;
            } else {
                cnt_consonant1 += 1;
                cnt_consonant2 += 1;
            }

            while size_vowel1 == 5 && cnt_consonant1 >= k {
                let out = (word.as_bytes()[left1] - b'a') as usize;
                if (VOWEL_MASK >> out) & 1 == 1 {
                    if cnt_vowel1[out] == 1 {
                        size_vowel1 -= 1;
                    }
                    cnt_vowel1[out] -= 1;
                } else {
                    cnt_consonant1 -= 1;
                }
                left1 += 1;
            }

            while size_vowel2 == 5 && cnt_consonant2 > k {
                let out = (word.as_bytes()[left2] - b'a') as usize;
                if (VOWEL_MASK >> out) & 1 == 1 {
                    if cnt_vowel2[out] == 1 {
                        size_vowel2 -= 1;
                    }
                    cnt_vowel2[out] -= 1;
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

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let line1 = lines.next().unwrap().unwrap();
    let parts: Vec<&str> = line1.split_whitespace().collect();
    let word_size: usize = parts[0].parse().unwrap();
    let k: usize = parts[2].parse().unwrap();
    let word = parts[1];

    let ans = Solution::count_of_substrings(word, k);
    println!("{}", ans);
}