use std::io::{self, BufRead};

fn count_of_substrings(word: &str, k: i32) -> i64 {
    const VOWEL_MASK: i64 = 1065233;
    let mut ans = 0i64;
    let mut cnt_vowel1 = [0; 26];
    let mut cnt_vowel2 = [0; 26];
    let mut size_vowel1 = 0;
    let mut size_vowel2 = 0;
    let mut cnt_consonant1 = 0;
    let mut cnt_consonant2 = 0;
    let mut left1 = 0;
    let mut left2 = 0;
    let length = word.len();

    for (i, c) in word.chars().enumerate() {
        let b = c as usize - 'a' as usize;

        // Update counts for current character
        if (VOWEL_MASK >> b) & 1 != 0 {
            // Vowel handling for window1
            if cnt_vowel1[b] == 0 {
                size_vowel1 += 1;
            }
            cnt_vowel1[b] += 1;

            // Vowel handling for window2
            if cnt_vowel2[b] == 0 {
                size_vowel2 += 1;
            }
            cnt_vowel2[b] += 1;
        } else {
            // Consonant handling
            cnt_consonant1 += 1;
            cnt_consonant2 += 1;
        }

        // Shrink window1 to maintain exactly 5 vowels and consonant count >= k
        while size_vowel1 == 5 && cnt_consonant1 >= k {
            let out_char = word.chars().nth(left1).unwrap();
            let out_b = out_char as usize - 'a' as usize;
            if (VOWEL_MASK >> out_b) & 1 != 0 {
                cnt_vowel1[out_b] -= 1;
                if cnt_vowel1[out_b] == 0 {
                    size_vowel1 -= 1;
                }
            } else {
                cnt_consonant1 -= 1;
            }
            left1 += 1;
        }

        // Shrink window2 to maintain exactly 5 vowels and consonant count > k
        while size_vowel2 == 5 && cnt_consonant2 > k {
            let out_char = word.chars().nth(left2).unwrap();
            let out_b = out_char as usize - 'a' as usize;
            if (VOWEL_MASK >> out_b) & 1 != 0 {
                cnt_vowel2[out_b] -= 1;
                if cnt_vowel2[out_b] == 0 {
                    size_vowel2 -= 1;
                }
            } else {
                cnt_consonant2 -= 1;
            }
            left2 += 1;
        }

        // Add valid substrings ending at current position
        ans += (left1 as i64 - left2 as i64);
    }

    ans
}

fn main() {
    let stdin = io::stdin();
    let mut tokens = stdin.lock().split_whitespace();
    let _word_size: usize = tokens.next().unwrap().parse().unwrap();
    let word = tokens.next().unwrap();
    let k: i32 = tokens.next().unwrap().parse().unwrap();

    println!("{}", count_of_substrings(&word, k));
}