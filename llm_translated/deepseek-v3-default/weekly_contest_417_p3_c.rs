use std::io::{self, BufRead};

fn count_of_substrings(word: &str, k: i32) -> i64 {
    const VOWEL_MASK: i64 = 1065233;
    let mut ans: i64 = 0;
    
    // Initialize vowel count arrays and related variables
    let mut cnt_vowel1 = [0; 26];
    let mut cnt_vowel2 = [0; 26];
    let mut size_vowel1 = 0; // Number of distinct vowels in window1
    let mut size_vowel2 = 0; // Number of distinct vowels in window2
    let mut cnt_consonant1 = 0;
    let mut cnt_consonant2 = 0;
    let mut left1 = 0;
    let mut left2 = 0;
    
    for (i, c) in word.chars().enumerate() {
        let b = (c as u8 - b'a') as usize;
        
        if ((VOWEL_MASK >> b) & 1) != 0 {
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
            let out = (word.chars().nth(left1).unwrap() as u8 - b'a') as usize;
            if ((VOWEL_MASK >> out) & 1) != 0 {
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
            let out = (word.chars().nth(left2).unwrap() as u8 - b'a') as usize;
            if ((VOWEL_MASK >> out) & 1) != 0 {
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

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the word size
    let word_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read the word
    let word = lines.next().unwrap().unwrap();
    
    // Read k
    let k: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Calculate and print the result
    println!("{}", count_of_substrings(&word, k));
}