use std::io::{self, BufRead};

struct Solution {
    vowel_mask: i32,
}

impl Solution {
    fn new() -> Self {
        Solution {
            vowel_mask: 1065233, // Bit mask for vowels (a, e, i, o, u)
        }
    }

    fn f(&self, word: &str, k: i32) -> i64 {
        let mut ans: i64 = 0;
        let mut cnt1 = vec![0; ('u' as usize) - ('a' as usize) + 1];
        let mut size1 = 0; // Number of vowel types
        let mut cnt2 = 0;  // Count of consonants
        let mut left = 0;
        
        let word_bytes: Vec<u8> = word.bytes().collect();
        
        for (right, &b_char) in word_bytes.iter().enumerate() {
            let b = (b_char - b'a') as usize;
            
            if (self.vowel_mask >> b) & 1 == 1 {
                if cnt1[b] == 0 {
                    size1 += 1;
                }
                cnt1[b] += 1;
            } else {
                cnt2 += 1;
            }
            
            while size1 == 5 && cnt2 >= k {
                let out = (word_bytes[left] - b'a') as usize;
                if (self.vowel_mask >> out) & 1 == 1 {
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

    pub fn count_of_substrings(&self, word: String, k: i32) -> i64 {
        self.f(&word, k) - self.f(&word, k + 1)
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read word size (though we don't actually use it in the solution)
    let _word_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read the word
    let word = lines.next().unwrap().unwrap();
    
    // Read k
    let k: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    let sol = Solution::new();
    println!("{}", sol.count_of_substrings(word, k));
}