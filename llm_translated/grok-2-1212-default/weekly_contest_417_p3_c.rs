// Problem: Weekly Contest 417 Problem 3

use std::io::{self, Read};

const VOWEL_MASK: u32 = 1065233;

fn count_of_substrings(word: &str, k: i32) -> i64 {
    let mut ans = 0;
    let mut cnt_vowel1 = [0; 26];
    let mut cnt_vowel2 = [0; 26];
    let mut size_vowel1 = 0;
    let mut size_vowel2 = 0;
    let mut cnt_consonant1 = 0;
    let mut cnt_consonant2 = 0;
    let mut left1 = 0;
    let mut left2 = 0;

    let word_bytes = word.as_bytes();
    let length = word_bytes.len();

    for i in 0..length {
        let b = (word_bytes[i] - b'a') as usize;

        if (VOWEL_MASK >> b) & 1 == 1 {
            // Update window1
            if cnt_vowel1[b] == 0 {
                size_vowel1 += 1;
            }
            cnt_vowel1[b] += 1;

            // Update window2
            if cnt_vowel2[b] == 0 {
                size_vowel2 += 1;
            }
            cnt_vowel2[b] += 1;
        } else {
            // It's a consonant
            cnt_consonant1 += 1;
            cnt_consonant2 += 1;
        }

        // Shrink window1 to maintain cnt_consonant1 >= k
        while size_vowel1 == 5 && cnt_consonant1 >= k {
            let out = (word_bytes[left1] - b'a') as usize;
            if (VOWEL_MASK >> out) & 1 == 1 {
                cnt_vowel1[out] -= 1;
                if cnt_vowel1[out] == 0 {
                    size_vowel1 -= 1;
                }
            } else {
                cnt_consonant1 -= 1;
            }
            left1 += 1;
        }

        // Shrink window2 to maintain cnt_consonant2 > k
        while size_vowel2 == 5 && cnt_consonant2 > k {
            let out = (word_bytes[left2] - b'a') as usize;
            if (VOWEL_MASK >> out) & 1 == 1 {
                cnt_vowel2[out] -= 1;
                if cnt_vowel2[out] == 0 {
                    size_vowel2 -= 1;
                }
            } else {
                cnt_consonant2 -= 1;
            }
            left2 += 1;
        }

        // Add the number of valid substrings ending at the current character
        ans += (left1 - left2) as i64;
    }

    ans
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    // Read word size
    let word_size: usize = lines.next().unwrap().parse().unwrap();

    // Read word
    let word = lines.next().unwrap();

    // Read k
    let k: i32 = lines.next().unwrap().parse().unwrap();

    // Calculate and print result
    let result = count_of_substrings(word, k);
    println!("{}", result);

    Ok(())
}