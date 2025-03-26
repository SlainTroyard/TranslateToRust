use std::io::{self, BufRead};

/// Count the number of substrings such that all 5 vowels a,e,i,o,u are present
/// and include at least k consonants within those substrings.
fn count_of_substrings(word: &str, k: i32) -> i64 {
    // This mask is used to check if a character is a vowel.
    // (vowel_mask >> b) & 1 != 0 will be true if the character is a vowel.
    const VOWEL_MASK: i64 = 1065233;

    let mut ans: i64 = 0;

    // Arrays tracking how many of each vowel is in the sliding windows
    let mut cnt_vowel1 = [0i32; 26];
    let mut cnt_vowel2 = [0i32; 26];

    // Number of distinct vowels in each sliding window
    let mut size_vowel1: i32 = 0;
    let mut size_vowel2: i32 = 0;

    // Count of consonants in each sliding window
    let mut cnt_consonant1: i32 = 0;
    let mut cnt_consonant2: i32 = 0;

    // Left pointers for the two sliding windows
    let mut left1: usize = 0;
    let mut left2: usize = 0;

    let length = word.len();

    for i in 0..length {
        let b = (word.as_bytes()[i] - b'a') as i32;

        // Check if current character is a vowel
        if ((VOWEL_MASK >> b) & 1) != 0 {
            // Update window1
            if cnt_vowel1[b as usize] == 0 {
                size_vowel1 += 1;
            }
            cnt_vowel1[b as usize] += 1;

            // Update window2
            if cnt_vowel2[b as usize] == 0 {
                size_vowel2 += 1;
            }
            cnt_vowel2[b as usize] += 1;
        } else {
            // It's a consonant
            cnt_consonant1 += 1;
            cnt_consonant2 += 1;
        }

        // Shrink window1 to maintain cnt_consonant1 >= k
        while size_vowel1 == 5 && cnt_consonant1 >= k {
            let out = (word.as_bytes()[left1] - b'a') as i32;
            if ((VOWEL_MASK >> out) & 1) != 0 {
                cnt_vowel1[out as usize] -= 1;
                if cnt_vowel1[out as usize] == 0 {
                    size_vowel1 -= 1;
                }
            } else {
                cnt_consonant1 -= 1;
            }
            left1 += 1;
        }

        // Shrink window2 to maintain cnt_consonant2 > k
        while size_vowel2 == 5 && cnt_consonant2 > k {
            let out = (word.as_bytes()[left2] - b'a') as i32;
            if ((VOWEL_MASK >> out) & 1) != 0 {
                cnt_vowel2[out as usize] -= 1;
                if cnt_vowel2[out as usize] == 0 {
                    size_vowel2 -= 1;
                }
            } else {
                cnt_consonant2 -= 1;
            }
            left2 += 1;
        }

        // Add the number of valid substrings ending at the current character
        ans += (left1 as i64) - (left2 as i64);
    }

    ans
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the size of the word
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let word_size_line = lines.next().ok_or("Missing input for word size")??;
    let word_size: usize = word_size_line.trim().parse()?;

    // Read the word itself
    let word_line = lines.next().ok_or("Missing input for word")??;
    let word = word_line.trim();

    // Read the integer k
    let k_line = lines.next().ok_or("Missing input for k")??;
    let k: i32 = k_line.trim().parse()?;

    // Since the problem statement expects exactly one line of output
    let result = count_of_substrings(word, k);
    println!("{}", result);

    Ok(())
}