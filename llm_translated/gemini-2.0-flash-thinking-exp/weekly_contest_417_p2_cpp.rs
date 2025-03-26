// Problem: Weekly Contest 417 Problem 2
use std::io;

struct Solution {}

impl Solution {
    const VOWEL_MASK: u32 = 1065233; // Binary representation of vowels 'aeiou'

    fn f(word: &str, k: i32) -> i64 {
        let mut ans: i64 = 0;
        let mut cnt1 = [0; 26]; // Count of each vowel
        let mut size1 = 0; // Number of distinct vowels
        let mut cnt2 = 0; // Count of consonants
        let mut left = 0;
        let word_chars: Vec<char> = word.chars().collect(); // Convert string to vector of chars for easier indexing

        for &b_char in &word_chars {
            let b = (b_char as u8 - b'a') as usize; // Convert char to index (0-25)

            if (Self::VOWEL_MASK >> b) & 1 != 0 { // Check if it's a vowel
                if cnt1[b] == 0 {
                    size1 += 1;
                }
                cnt1[b] += 1;
            } else {
                cnt2 += 1;
            }

            while size1 == 5 && cnt2 >= k {
                let out_char = word_chars[left];
                let out = (out_char as u8 - b'a') as usize;
                if (Self::VOWEL_MASK >> out) & 1 != 0 { // Check if it's a vowel
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

fn main() {
    let mut word_size_str = String::new();
    io::stdin().read_line(&mut word_size_str).expect("Failed to read line");
    let _word_size: i32 = word_size_str.trim().parse().expect("Invalid input"); // wordSize is not used

    let mut word = String::new();
    io::stdin().read_line(&mut word).expect("Failed to read line");
    let word = word.trim();

    let mut k_str = String::new();
    io::stdin().read_line(&mut k_str).expect("Failed to read line");
    let k: i32 = k_str.trim().parse().expect("Invalid input");

    let sol = Solution {};
    println!("{}", sol.count_of_substrings(word.to_string(), k));
}