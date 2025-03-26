use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn count_of_substrings(word: &str, k: usize) -> i64 {
        const VOWEL_MASK: u32 = 1065233; // Used to determine if a character is a vowel
        let mut ans: i64 = 0;

        // Arrays to store counts of vowels and their uniqueness
        let mut cnt_vowel1 = [0; 26];
        let mut cnt_vowel2 = [0; 26];
        let mut size_vowel1 = 0;
        let mut size_vowel2 = 0;

        // Counters for consonants
        let mut cnt_consonant1 = 0;
        let mut cnt_consonant2 = 0;

        // Sliding window pointers
        let mut left1 = 0;
        let mut left2 = 0;

        let word_bytes = word.as_bytes(); // Access bytes for easier char manipulation

        for &b in word_bytes {
            let b = (b - b'a') as usize; // Convert character to index (0-25)
            if (VOWEL_MASK >> b) & 1 == 1 {
                // Check if it's a vowel
                if cnt_vowel1[b] == 0 {
                    size_vowel1 += 1;
                }
                cnt_vowel1[b] += 1;

                if cnt_vowel2[b] == 0 {
                    size_vowel2 += 1;
                }
                cnt_vowel2[b] += 1;
            } else {
                // It's a consonant
                cnt_consonant1 += 1;
                cnt_consonant2 += 1;
            }

            // Adjust `left1` to maintain the condition `size_vowel1 == 5 && cnt_consonant1 >= k`
            while size_vowel1 == 5 && cnt_consonant1 >= k {
                let out = (word_bytes[left1] - b'a') as usize;
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

            // Adjust `left2` to maintain the condition `size_vowel2 == 5 && cnt_consonant2 > k`
            while size_vowel2 == 5 && cnt_consonant2 > k {
                let out = (word_bytes[left2] - b'a') as usize;
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

            // Add to the result the number of valid substrings ending at this position
            ans += (left1 - left2) as i64;
        }

        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the word (in C++ this was used but isn't required for Rust, just skip it)
    let _word_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read the word string
    let word = lines.next().unwrap().unwrap().trim().to_string();

    // Read the k value
    let k: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Calculate the answer using the Solution struct
    let solution = Solution;
    let ans = solution.count_of_substrings(&word, k);

    // Print the result
    println!("{}", ans);
}