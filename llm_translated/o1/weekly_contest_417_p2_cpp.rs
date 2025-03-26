use std::io::{self, Read};

/// A struct to replicate the C++ Solution class
struct Solution {
    /// Bitmask used to identify vowels (matching the original C++ VOWEL_MASK)
    vowel_mask: u64,
}

impl Solution {
    /// Constructs a new Solution with the same vowel_mask as in the C++ code
    fn new() -> Self {
        Self {
            vowel_mask: 1065233,
        }
    }

    /// This function mirrors the C++ function f(string& word, int k).
    /// It calculates the number of valid substrings based on vowel/consonant constraints.
    fn f(&self, word: &str, k: i32) -> i64 {
        let mut ans: i64 = 0;
        // Array to hold counts of vowels up to 'u' (index 20)
        let mut cnt1 = [0_i32; 21];
        // Distinct vowel count
        let mut size1 = 0;
        // Count of consonants
        let mut cnt2 = 0;
        // Left pointer for our sliding window
        let mut left = 0_usize;
        // Iterate through the word by bytes
        let bytes = word.as_bytes();

        for &c in bytes.iter() {
            // Convert character to 0-based index ('a' -> 0, 'b' -> 1, etc.)
            let b = (c - b'a') as usize;
            // Check if this character is a vowel using vowel_mask
            if ((self.vowel_mask >> b) & 1) == 1 {
                // If it's the first time we see this vowel, increment distinct vowel count
                if cnt1[b] == 0 {
                    size1 += 1;
                }
                cnt1[b] += 1;
            } else {
                // Otherwise, it's a consonant
                cnt2 += 1;
            }

            // While we have all 5 vowels and at least k consonants, shrink window from the left
            while size1 == 5 && cnt2 >= k {
                let out = (bytes[left] - b'a') as usize;
                if ((self.vowel_mask >> out) & 1) == 1 {
                    cnt1[out] -= 1;
                    // If we've removed this vowel completely, reduce distinct vowel count
                    if cnt1[out] == 0 {
                        size1 -= 1;
                    }
                } else {
                    // If removing a consonant, decrement consonant count
                    cnt2 -= 1;
                }
                left += 1;
            }

            // Add the number of valid substrings ending here
            ans += left as i64;
        }

        ans
    }

    /// Mirrors the C++ countOfSubstrings(string word, int k) function.
    /// Returns the difference f(word, k) - f(word, k + 1).
    fn count_of_substrings(&self, word: &str, k: i32) -> i64 {
        self.f(word, k) - self.f(word, k + 1)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read all input into a single string
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Tokenize the input to replicate C++'s parsing with cin >> ...
    let mut tokens = input.split_whitespace();

    // 1) Read wordSize (ignored like in the original C++ code, but we must match the format)
    let word_size: usize = tokens.next().ok_or("Missing wordSize")?.parse()?;

    // 2) Read the word
    let word = tokens.next().ok_or("Missing word")?.to_string();

    // 3) Read k
    let k: i32 = tokens.next().ok_or("Missing k")?.parse()?;

    // Create an instance of the solution and compute the result
    let sol = Solution::new();
    let result = sol.count_of_substrings(&word, k);

    // Print the result (exact single-line output format)
    println!("{}", result);

    Ok(())
}