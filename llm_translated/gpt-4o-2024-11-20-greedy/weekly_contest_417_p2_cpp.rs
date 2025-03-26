use std::io::{self, BufRead};

struct Solution {
    const VOWEL_MASK: u32,
}

impl Solution {
    fn new() -> Self {
        Self {
            VOWEL_MASK: 1065233, // Binary representation of vowels: aeiou
        }
    }

    fn f(&self, word: &str, k: usize) -> i64 {
        let mut ans = 0;
        let mut cnt1 = vec![0; ('u' as usize - 'a' as usize + 1)];
        let mut size1 = 0; // Number of distinct vowels
        let mut cnt2 = 0; // Count of non-vowel characters
        let mut left = 0;

        for b in word.bytes() {
            let b = (b - b'a') as usize;
            if (self.VOWEL_MASK >> b) & 1 == 1 {
                if cnt1[b] == 0 {
                    size1 += 1;
                }
                cnt1[b] += 1;
            } else {
                cnt2 += 1;
            }

            while size1 == 5 && cnt2 >= k {
                let out = (word.as_bytes()[left] - b'a') as usize;
                if (self.VOWEL_MASK >> out) & 1 == 1 {
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

    fn count_of_substrings(&self, word: &str, k: usize) -> i64 {
        self.f(word, k) - self.f(word, k + 1)
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read word size (not used in the logic, but part of the input format)
    let word_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read the word
    let word = lines.next().unwrap().unwrap();

    // Read k
    let k: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    let solution = Solution::new();
    let result = solution.count_of_substrings(&word, k);

    // Output the result
    println!("{}", result);
}