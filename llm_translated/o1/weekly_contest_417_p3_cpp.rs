use std::error::Error;
use std::io::{self, Read};

/// Count the number of valid substrings according to the given logic:
/// - We track two sliding windows (left1, left2).
/// - A substring is considered when it has exactly 5 distinct vowels
///   and at least k consonants (handled via window constraints).
fn count_of_substrings(word: &str, k: i32) -> i64 {
    // This bitmask marks the positions of 'a', 'e', 'i', 'o', 'u' (relative to 'a').
    //  For a character c, if (VOWEL_MASK >> (c - 'a')) & 1 == 1, then it's a vowel.
    const VOWEL_MASK: u32 = 1065233;

    let mut ans: i64 = 0;

    // We only need to handle up to 'u', which is 20 characters away from 'a'.
    // Hence, arrays of size 21 suffice to record vowel frequencies.
    let mut cnt_vowel1 = [0; 21];
    let mut cnt_vowel2 = [0; 21];

    let mut size_vowel1 = 0; // number of distinct vowels in window1
    let mut size_vowel2 = 0; // number of distinct vowels in window2

    let mut cnt_consonant1 = 0;
    let mut cnt_consonant2 = 0;

    let mut left1 = 0;
    let mut left2 = 0;

    // Process the string as bytes for quick arithmetic.
    let bytes = word.as_bytes();

    for (right, &ch) in bytes.iter().enumerate() {
        // Convert character to 0-based index relative to 'a'
        let b = ch - b'a';

        // Check if it's a vowel via bitmask
        if (VOWEL_MASK >> b) & 1 == 1 {
            // If this vowel hasn't appeared yet in the respective window, increase distinct count
            if cnt_vowel1[b as usize] == 0 {
                size_vowel1 += 1;
            }
            cnt_vowel1[b as usize] += 1;

            if cnt_vowel2[b as usize] == 0 {
                size_vowel2 += 1;
            }
            cnt_vowel2[b as usize] += 1;
        } else {
            // Consonant
            cnt_consonant1 += 1;
            cnt_consonant2 += 1;
        }

        // Shrink the first window while we have 5 distinct vowels AND at least k consonants
        while size_vowel1 == 5 && cnt_consonant1 >= k {
            let out = bytes[left1] - b'a';
            if (VOWEL_MASK >> out) & 1 == 1 {
                cnt_vowel1[out as usize] -= 1;
                if cnt_vowel1[out as usize] == 0 {
                    size_vowel1 -= 1;
                }
            } else {
                cnt_consonant1 -= 1;
            }
            left1 += 1;
        }

        // Shrink the second window while we have 5 distinct vowels AND more than k consonants
        while size_vowel2 == 5 && cnt_consonant2 > k {
            let out = bytes[left2] - b'a';
            if (VOWEL_MASK >> out) & 1 == 1 {
                cnt_vowel2[out as usize] -= 1;
                if cnt_vowel2[out as usize] == 0 {
                    size_vowel2 -= 1;
                }
            } else {
                cnt_consonant2 -= 1;
            }
            left2 += 1;
        }

        // Count the number of valid substrings ending at 'right'
        // which are bounded between the two sliding window positions.
        ans += (left1 - left2) as i64;
    }

    ans
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read all input from stdin
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    // Tokenize the input to extract words/numbers in the correct order:
    let mut tokens = buffer.split_whitespace();

    // 1) word_size (not directly needed beyond reading it, but must parse to match the C++ logic)
    let word_size = tokens
        .next()
        .ok_or("Missing 'word_size' in input")?
        .parse::<usize>()?;

    // 2) word (string)
    let word = tokens
        .next()
        .ok_or("Missing 'word' in input")?
        .to_string();

    // 3) k (integer)
    let k = tokens
        .next()
        .ok_or("Missing 'k' in input")?
        .parse::<i32>()?;

    // Compute the answer
    let ans = count_of_substrings(&word, k);

    // Print exactly as the C++ code does
    println!("{}", ans);

    Ok(())
}