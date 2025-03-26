use std::io;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn count_of_substrings(word: String, k: i32) -> i64 {
        const VOWEL_MASK: u32 = 1065233; // Binary 1000001000001000001
        let mut ans: i64 = 0;
        let mut cnt_vowel1 = [0; 26];
        let mut cnt_vowel2 = [0; 26];
        let mut size_vowel1 = 0;
        let mut size_vowel2 = 0; // 元音种类数
        let mut cnt_consonant1 = 0;
        let mut cnt_consonant2 = 0;
        let mut left1 = 0;
        let mut left2 = 0;
        let word_bytes = word.as_bytes(); // Iterate over bytes for efficiency

        for &b in word_bytes.iter() {
            let b_index = (b - b'a') as usize; // Convert byte to index 0-25
            if (VOWEL_MASK >> b_index) & 1 != 0 { // Check if it's a vowel
                if cnt_vowel1[b_index] == 0 {
                    size_vowel1 += 1;
                }
                cnt_vowel1[b_index] += 1;

                if cnt_vowel2[b_index] == 0 {
                    size_vowel2 += 1;
                }
                cnt_vowel2[b_index] += 1;
            } else {
                cnt_consonant1 += 1;
                cnt_consonant2 += 1;
            }

            while size_vowel1 == 5 && cnt_consonant1 >= k {
                let out_index = (word_bytes[left1] - b'a') as usize;
                if (VOWEL_MASK >> out_index) & 1 != 0 {
                    cnt_vowel1[out_index] -= 1;
                    if cnt_vowel1[out_index] == 0 {
                        size_vowel1 -= 1;
                    }
                } else {
                    cnt_consonant1 -= 1;
                }
                left1 += 1;
            }

            while size_vowel2 == 5 && cnt_consonant2 > k {
                let out_index = (word_bytes[left2] - b'a') as usize;
                if (VOWEL_MASK >> out_index) & 1 != 0 {
                    cnt_vowel2[out_index] -= 1;
                    if cnt_vowel2[out_index] == 0 {
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

fn main() {
    let mut word_size_str = String::new();
    io::stdin().read_line(&mut word_size_str).unwrap();
    let mut parts = word_size_str.trim().split_whitespace();
    let word_size: usize = parts.next().unwrap().parse().unwrap(); // Parse word_size, but it's not actually used in the algorithm
    let word: String = parts.next().unwrap().parse().unwrap();

    let mut k_str = String::new();
    io::stdin().read_line(&mut k_str).unwrap();
    let k: i32 = k_str.trim().parse().unwrap();

    let ans = Solution {}.count_of_substrings(word, k);
    println!("{}", ans);
}