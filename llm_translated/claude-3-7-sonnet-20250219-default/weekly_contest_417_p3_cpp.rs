use std::io::{self, BufRead};

// Problem: Weekly Contest 417 Problem 3
struct Solution;

impl Solution {
    pub fn count_of_substrings(word: String, k: i32) -> i64 {
        const VOWEL_MASK: i32 = 1065233; // Bit mask for vowels (a, e, i, o, u)
        let word_bytes = word.as_bytes();
        let mut ans: i64 = 0;
        
        // Arrays to track vowel counts for the two different sliding windows
        let mut cnt_vowel1 = [0; (b'u' - b'a' + 1) as usize];
        let mut cnt_vowel2 = [0; (b'u' - b'a' + 1) as usize];
        
        let mut size_vowel1 = 0; // Count of distinct vowels in window 1
        let mut size_vowel2 = 0; // Count of distinct vowels in window 2
        
        let mut cnt_consonant1 = 0; // Count of consonants in window 1
        let mut cnt_consonant2 = 0; // Count of consonants in window 2
        
        let mut left1 = 0;
        let mut left2 = 0;
        
        for (i, &b) in word_bytes.iter().enumerate() {
            let b_idx = (b - b'a') as usize;
            
            // Check if the character is a vowel using the bit mask
            if (VOWEL_MASK >> b_idx) & 1 == 1 {
                // Update vowel counters for both windows
                if cnt_vowel1[b_idx] == 0 {
                    size_vowel1 += 1;
                }
                cnt_vowel1[b_idx] += 1;
                
                if cnt_vowel2[b_idx] == 0 {
                    size_vowel2 += 1;
                }
                cnt_vowel2[b_idx] += 1;
            } else {
                // Update consonant counters for both windows
                cnt_consonant1 += 1;
                cnt_consonant2 += 1;
            }
            
            // Adjust window 1: handles the case where we have all vowels and at least k consonants
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
            
            // Adjust window 2: handles the case where we have all vowels and more than k consonants
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
            
            // Count valid substrings ending at current position
            ans += (left1 as i64) - (left2 as i64);
        }
        
        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read word_size (though not used directly in the solution)
    let word_size = lines.next().unwrap().unwrap().trim().parse::<usize>().unwrap();
    
    // Read the word
    let word = lines.next().unwrap().unwrap();
    
    // Read k
    let k = lines.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
    
    // Calculate and print the result
    let ans = Solution::count_of_substrings(word, k);
    println!("{}", ans);
}