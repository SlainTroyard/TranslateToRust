use std::io::{self, BufRead, Write};

const VOWEL_MASK: u32 = 1065233;

struct Solution;

impl Solution {
    fn f(word: &str, k: usize) -> i64 {
        let mut ans = 0;
        let mut cnt1 = [0; 'u' as usize - 'a' as usize + 1];
        let mut size1 = 0; // Number of distinct vowel types
        let mut cnt2 = 0;
        let mut left = 0;
        for (right, b) in word.chars().enumerate() {
            let b = b as u8 - b'a';
            if (VOWEL_MASK >> b & 1) != 0 {
                if cnt1[b as usize] == 0 {
                    size1 += 1;
                }
                cnt1[b as usize] += 1;
            } else {
                cnt2 += 1;
            }
            while size1 == 5 && cnt2 >= k {
                let out = word.as_bytes()[left] - b'a';
                if (VOWEL_MASK >> out & 1) != 0 {
                    if cnt1[out as usize] == 1 {
                        size1 -= 1;
                    }
                    cnt1[out as usize] -= 1;
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
    let mut stdout = io::stdout();
    let mut buffer = String::new();

    // Read the word size (not used in this implementation)
    stdin.lock().read_line(&mut buffer).unwrap();
    buffer.clear();

    // Read the word
    stdin.lock().read_line(&mut buffer).unwrap();
    let word = buffer.trim();
    buffer.clear();

    // Read the value of k
    stdin.lock().read_line(&mut buffer).unwrap();
    let k: usize = buffer.trim().parse().expect("Failed to parse k");

    let sol = Solution;
    let result = sol.count_of_substrings(word, k);

    writeln!(stdout, "{}", result).unwrap();
}