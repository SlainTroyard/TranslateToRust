use std::io::{self, BufRead};

const VOWEL_MASK: u32 = 1065233;

struct Solution;

impl Solution {
    fn f(word: &str, k: usize) -> i64 {
        let mut ans = 0;
        let mut cnt1 = [0; 'u' as usize - 'a' as usize + 1];
        let mut size1 = 0; // Number of vowel types
        let mut cnt2 = 0;
        let mut left = 0;
        
        for (right, &b) in word.as_bytes().iter().enumerate() {
            let b = *b as usize - 'a' as usize;
            if VOWEL_MASK >> b & 1 == 1 {
                if cnt1[b] == 0 {
                    size1 += 1;
                }
                cnt1[b] += 1;
            } else {
                cnt2 += 1;
            }
            
            while size1 == 5 && cnt2 >= k {
                let out = word.as_bytes()[left] as usize - 'a' as usize;
                if VOWEL_MASK >> out & 1 == 1 {
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

    pub fn count_of_substrings(word: &str, k: usize) -> i64 {
        Self::f(word, k) - Self::f(word, k + 1)
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let word_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let word: String = lines.next().unwrap().unwrap();
    let k: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    let sol = Solution;
    println!("{}", sol.count_of_substrings(&word, k));
}