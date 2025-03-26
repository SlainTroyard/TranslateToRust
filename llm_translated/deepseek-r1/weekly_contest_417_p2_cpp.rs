use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn count_of_substrings(word: String, k: i32) -> i64 {
        const VOWEL_MASK: i32 = 1065233; // Represents vowels a, e, i, o, u via bitmask

        // Helper function to count valid substrings with at least k consonants and all vowels
        fn f(word: &str, k: i32) -> i64 {
            let word_bytes = word.as_bytes();
            let mut cnt1 = [0; 21]; // Tracks counts of vowels (indices 0-20)
            let mut size1 = 0;      // Number of unique vowels in current window
            let mut cnt2 = 0;       // Consonant count in current window
            let mut left = 0;       // Left pointer of sliding window
            let mut ans = 0i64;

            for &c in word_bytes {
                let b = (c - b'a') as i32;

                if (VOWEL_MASK >> b) & 1 != 0 {
                    // Current character is a vowel
                    if cnt1[b as usize] == 0 {
                        size1 += 1;
                    }
                    cnt1[b as usize] += 1;
                } else {
                    // Current character is a consonant
                    cnt2 += 1;
                }

                // Shrink window while maintaining validity (all vowels and >=k consonants)
                while size1 == 5 && cnt2 >= k {
                    let left_char = word_bytes[left];
                    let left_b = (left_char - b'a') as i32;

                    if (VOWEL_MASK >> left_b) & 1 != 0 {
                        // Remove left vowel from window
                        cnt1[left_b as usize] -= 1;
                        if cnt1[left_b as usize] == 0 {
                            size1 -= 1;
                        }
                    } else {
                        // Remove left consonant from window
                        cnt2 -= 1;
                    }
                    left += 1;
                }

                // All valid substrings ending at current position start before left
                ans += left as i64;
            }

            ans
        }

        // Calculate result using inclusion-exclusion principle
        f(&word, k) - f(&word, k + 1)
    }
}

fn main() {
    // Read input in the same format as original C++ code (whitespace separated tokens)
    let mut tokens = io::stdin()
        .lock()
        .lines()
        .flat_map(|line| line.unwrap().split_whitespace().map(String::from).collect::<Vec<_>>());

    // Input parsing (word_size is read but not used)
    let _word_size: usize = tokens.next().unwrap().parse().unwrap();
    let word = tokens.next().unwrap();
    let k: i32 = tokens.next().unwrap().parse().unwrap();

    // Compute and print result
    println!("{}", Solution::count_of_substrings(word, k));
}