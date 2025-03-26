use std::io;

struct Solution;

impl Solution {
    const VOWEL_MASK: i32 = 1065233;

    fn f(&self, word: &str, k: i32) -> i64 {
        let mut ans: i64 = 0;
        let mut cnt1 = [0; ('u' - 'a' + 1) as usize];
        let mut size1 = 0; // Number of vowel types
        let mut cnt2 = 0;
        let mut left = 0;
        let word_bytes = word.as_bytes();
        
        for (right, &b) in word_bytes.iter().enumerate() {
            let b_idx = (b - b'a') as usize;
            if (Self::VOWEL_MASK >> b_idx & 1) != 0 {
                if cnt1[b_idx] == 0 {
                    size1 += 1;
                }
                cnt1[b_idx] += 1;
            } else {
                cnt2 += 1;
            }
            
            while size1 == 5 && cnt2 >= k {
                let out = (word_bytes[left] - b'a') as usize;
                if (Self::VOWEL_MASK >> out & 1) != 0 {
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
    let mut input = String::new();
    
    // Read word size
    io::stdin().read_line(&mut input)?;
    let _word_size: usize = input.trim().parse().expect("Invalid word size");
    
    // Read word
    input.clear();
    io::stdin().read_line(&mut input)?;
    let word = input.trim().to_string();
    
    // Read k
    input.clear();
    io::stdin().read_line(&mut input)?;
    let k: i32 = input.trim().parse().expect("Invalid k");
    
    let sol = Solution;
    println!("{}", sol.count_of_substrings(word, k));
    
    Ok(())
}