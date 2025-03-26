use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn count_of_substrings(word: String, k: i32) -> i64 {
        const VOWEL_MASK: i32 = 1065233; // Bit mask for vowels
        let mut ans: i64 = 0;
        let word_bytes = word.as_bytes();
        
        // Arrays to track vowel counts
        let mut cnt_vowel1 = [0; (b'u' - b'a' + 1) as usize];
        let mut cnt_vowel2 = [0; (b'u' - b'a' + 1) as usize];
        
        let mut size_vowel1 = 0; // Number of different vowels
        let mut size_vowel2 = 0;
        let mut cnt_consonant1 = 0;
        let mut cnt_consonant2 = 0;
        let mut left1 = 0;
        let mut left2 = 0;
        
        for (right, &b) in word_bytes.iter().enumerate() {
            let idx = (b - b'a') as usize;
            
            if (VOWEL_MASK >> idx) & 1 == 1 {
                // Current character is a vowel
                if cnt_vowel1[idx] == 0 {
                    size_vowel1 += 1;
                }
                cnt_vowel1[idx] += 1;
                
                if cnt_vowel2[idx] == 0 {
                    size_vowel2 += 1;
                }
                cnt_vowel2[idx] += 1;
            } else {
                // Current character is a consonant
                cnt_consonant1 += 1;
                cnt_consonant2 += 1;
            }
            
            // Adjust left1 window to maintain the condition
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
            
            // Adjust left2 window to maintain the condition
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
            
            ans += (left1 - left2) as i64;
        }
        
        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read word_size and word
    let word_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let word = lines.next().unwrap().unwrap();
    
    // Read k
    let k: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    let ans = Solution::count_of_substrings(word, k);
    println!("{}", ans);
}