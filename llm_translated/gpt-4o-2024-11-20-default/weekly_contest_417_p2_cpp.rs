use std::io::{self, Read};

struct Solution {
    vowel_mask: u32,
}

impl Solution {
    fn new() -> Self {
        Self {
            vowel_mask: 1065233, // Binary mask for vowels (a, e, i, o, u)
        }
    }

    fn f(&self, word: &str, k: usize) -> i64 {
        let mut ans = 0i64;
        let mut cnt1 = [0i32; ('u' as u8 - 'a' as u8 + 1) as usize]; // Frequency of vowels
        let mut size1 = 0; // Number of vowel types
        let mut cnt2 = 0; // Count of non-vowel characters
        let mut left = 0; // Sliding window left index

        for &b in word.as_bytes() {
            let b = (b - b'a') as usize; // Normalize char to 0-based index
            if (self.vowel_mask >> b) & 1 != 0 {
                // If the character is a vowel
                if cnt1[b] == 0 {
                    size1 += 1; // New vowel type encountered
                }
                cnt1[b] += 1;
            } else {
                cnt2 += 1; // Non-vowel character
            }

            while size1 == 5 && cnt2 >= k {
                let out = (word.as_bytes()[left] - b'a') as usize; // Normalize char at left
                if (self.vowel_mask >> out) & 1 != 0 {
                    // If the character at left is a vowel
                    cnt1[out] -= 1;
                    if cnt1[out] == 0 {
                        size1 -= 1; // Vowel type removed
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
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.trim().lines();

    let word_size: usize = lines.next().unwrap().parse().unwrap(); // Word size (not used explicitly)
    let word = lines.next().unwrap().to_string(); // Word string
    let k: usize = lines.next().unwrap().parse().unwrap(); // Parameter k

    // Solve the problem
    let solution = Solution::new();
    let result = solution.count_of_substrings(&word, k);

    // Output the result to stdout
    println!("{}", result);
}