use std::io::{self, BufRead};
use std::collections::HashMap;

fn count_of_substrings(word: &str, k: usize) -> i64 {
    const VOWEL_MASK: u32 = 1065233; // Binary representation of vowels (a, e, i, o, u)
    let mut ans = 0;

    // Initialize vowel count maps and related variables
    let mut cnt_vowel1: HashMap<char, usize> = HashMap::new();
    let mut cnt_vowel2: HashMap<char, usize> = HashMap::new();
    let mut size_vowel1 = 0;
    let mut size_vowel2 = 0; // Number of distinct vowels in window1 and window2
    let mut cnt_consonant1 = 0;
    let mut cnt_consonant2 = 0;
    let mut left1 = 0;
    let mut left2 = 0;

    let length = word.len();
    let word_chars: Vec<char> = word.chars().collect();

    for (i, &ch) in word_chars.iter().enumerate() {
        let b = (ch as u8 - b'a') as u32;

        if (VOWEL_MASK >> b) & 1 != 0 {
            // Update window1
            *cnt_vowel1.entry(ch).or_insert(0) += 1;
            if cnt_vowel1[&ch] == 1 {
                size_vowel1 += 1;
            }
            // Update window2
            *cnt_vowel2.entry(ch).or_insert(0) += 1;
            if cnt_vowel2[&ch] == 1 {
                size_vowel2 += 1;
            }
        } else {
            // It's a consonant
            cnt_consonant1 += 1;
            cnt_consonant2 += 1;
        }

        // Shrink window1 to maintain cnt_consonant1 >= k
        while size_vowel1 == 5 && cnt_consonant1 >= k {
            let out = word_chars[left1];
            let out_b = (out as u8 - b'a') as u32;
            if (VOWEL_MASK >> out_b) & 1 != 0 {
                if let Some(count) = cnt_vowel1.get_mut(&out) {
                    *count -= 1;
                    if *count == 0 {
                        size_vowel1 -= 1;
                    }
                }
            } else {
                cnt_consonant1 -= 1;
            }
            left1 += 1;
        }

        // Shrink window2 to maintain cnt_consonant2 > k
        while size_vowel2 == 5 && cnt_consonant2 > k {
            let out = word_chars[left2];
            let out_b = (out as u8 - b'a') as u32;
            if (VOWEL_MASK >> out_b) & 1 != 0 {
                if let Some(count) = cnt_vowel2.get_mut(&out) {
                    *count -= 1;
                    if *count == 0 {
                        size_vowel2 -= 1;
                    }
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

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read word size (not used in Rust implementation)
    let word_size: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Read the word
    let word = lines.next().unwrap().unwrap();

    // Read k
    let k: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Calculate and print the result
    let result = count_of_substrings(&word, k);
    println!("{}", result);
}