use std::io::{self, BufRead};

struct Solution {
    // Vowel mask: a (1), e (4), i (8), o (14), u (20) positions set to 1
    // Binary: 00000000 00100001 00000100 00010001
    vowel_mask: i32,
}

impl Solution {
    fn new() -> Self {
        Solution {
            vowel_mask: 1065233,
        }
    }

    fn f(&self, word: &str, k: i32) -> i64 {
        let mut ans: i64 = 0;
        let mut cnt1 = vec![0; ('u' - 'a' + 1) as usize];
        let mut size1 = 0; // Count of vowel types
        let mut cnt2 = 0; // Count of consonants
        let mut left = 0;
        let word_bytes = word.as_bytes();

        for (right, &ch) in word_bytes.iter().enumerate() {
            let b = (ch - b'a') as usize;
            
            // Check if current character is a vowel
            if (self.vowel_mask >> b) & 1 == 1 {
                if cnt1[b] == 0 {
                    size1 += 1;
                }
                cnt1[b] += 1;
            } else {
                cnt2 += 1;
            }

            // Shrink window while we have all vowels and enough consonants
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

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read word size (not used in this implementation but kept for compatibility)
    let _word_size: usize = lines.next().unwrap()?.trim().parse().expect("Failed to parse word size");
    
    // Read word
    let word = lines.next().unwrap()?.trim().to_string();
    
    // Read k
    let k: i32 = lines.next().unwrap()?.trim().parse().expect("Failed to parse k");
    
    let sol = Solution::new();
    println!("{}", sol.count_of_substrings(word, k));
    
    Ok(())
}