use std::io;

struct Solution;

impl Solution {
    fn count_of_substrings(word: String, k: i32) -> i64 {
        // Bit mask for vowels (a, e, i, o, u)
        // 1065233 = 2^20 + 2^14 + 2^8 + 2^4 + 2^0 = (a, e, i, o, u) positions
        const VOWEL_MASK: i32 = 1065233;
        
        let mut ans: i64 = 0;
        let mut cnt_vowel1 = vec![0; ('u' as usize) - ('a' as usize) + 1];
        let mut cnt_vowel2 = vec![0; ('u' as usize) - ('a' as usize) + 1];
        
        // Number of unique vowels in each window
        let mut size_vowel1 = 0;
        let mut size_vowel2 = 0;
        
        // Number of consonants in each window
        let mut cnt_consonant1 = 0;
        let mut cnt_consonant2 = 0;
        
        // Left pointer for each window
        let mut left1 = 0;
        let mut left2 = 0;
        
        let word_bytes = word.as_bytes();
        
        for (i, &c) in word_bytes.iter().enumerate() {
            let b = (c - b'a') as usize;
            
            // Check if current character is a vowel
            if (VOWEL_MASK >> b) & 1 == 1 {
                if cnt_vowel1[b] == 0 {
                    size_vowel1 += 1;
                }
                cnt_vowel1[b] += 1;
                
                if cnt_vowel2[b] == 0 {
                    size_vowel2 += 1;
                }
                cnt_vowel2[b] += 1;
            } else {
                cnt_consonant1 += 1;
                cnt_consonant2 += 1;
            }
            
            // Adjust window 1: contains 5 unique vowels and at least k consonants
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
            
            // Adjust window 2: contains 5 unique vowels and more than k consonants
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
            ans += (left1 - left2) as i64;
        }
        
        ans
    }
}

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    
    let word_size: usize = parts[0].parse().expect("Failed to parse word_size");
    let k: i32 = parts[1].parse().expect("Failed to parse k");
    
    let mut word = String::new();
    io::stdin().read_line(&mut word).expect("Failed to read word");
    let word = word.trim().to_string();
    
    // Calculate and print result
    let ans = Solution::count_of_substrings(word, k);
    println!("{}", ans);
}