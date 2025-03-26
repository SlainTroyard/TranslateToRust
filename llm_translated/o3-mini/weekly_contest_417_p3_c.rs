use std::io::{self, BufRead};

/// This function calculates the count of substrings as defined in the problem statement.
/// It replicates the logic from the C code by maintaining two sliding windows.
fn count_of_substrings(word: &str, k: i32) -> i64 {
    // The vowel mask constant.
    const VOWEL_MASK: u64 = 1065233;
    
    // The answer to be computed
    let mut ans: i64 = 0;
    
    // Frequency arrays for the vowels (for two sliding windows).
    let mut cnt_vowel1 = [0; 26];
    let mut cnt_vowel2 = [0; 26];
    
    // Number of distinct vowels present in each of the two windows.
    let mut size_vowel1 = 0;
    let mut size_vowel2 = 0;
    
    // Counts for consonants in each window.
    let mut cnt_consonant1 = 0;
    let mut cnt_consonant2 = 0;
    
    // Left pointers for the sliding windows.
    let mut left1 = 0;
    let mut left2 = 0;
    
    // Convert the string to a byte array for processing.
    let bytes = word.as_bytes();
    let length = bytes.len();
    
    // Iterate over each character in the word.
    for i in 0..length {
        // Convert character from ASCII to index (0-based).
        let b = (bytes[i] - b'a') as usize;
        
        // Check if the current character is a vowel.
        if ((VOWEL_MASK >> b) & 1) != 0 {
            // Update window1 for vowels.
            if cnt_vowel1[b] == 0 {
                size_vowel1 += 1;
            }
            cnt_vowel1[b] += 1;
            
            // Update window2 for vowels.
            if cnt_vowel2[b] == 0 {
                size_vowel2 += 1;
            }
            cnt_vowel2[b] += 1;
        } else {
            // If it's a consonant, update the consonant counts for both windows.
            cnt_consonant1 += 1;
            cnt_consonant2 += 1;
        }
        
        // Shrink window1 to maintain the condition: cnt_consonant1 >= k.
        // This loop moves the left1 pointer until the condition is violated.
        while size_vowel1 == 5 && cnt_consonant1 >= k {
            let out = (bytes[left1] - b'a') as usize;
            if ((VOWEL_MASK >> out) & 1) != 0 {
                cnt_vowel1[out] -= 1;
                if cnt_vowel1[out] == 0 {
                    size_vowel1 -= 1;
                }
            } else {
                cnt_consonant1 -= 1;
            }
            left1 += 1;
        }
        
        // Shrink window2 to maintain the condition: cnt_consonant2 > k.
        while size_vowel2 == 5 && cnt_consonant2 > k {
            let out = (bytes[left2] - b'a') as usize;
            if ((VOWEL_MASK >> out) & 1) != 0 {
                cnt_vowel2[out] -= 1;
                if cnt_vowel2[out] == 0 {
                    size_vowel2 -= 1;
                }
            } else {
                cnt_consonant2 -= 1;
            }
            left2 += 1;
        }
        
        // The difference between left1 and left2 represents the number of valid substrings
        // ending at the current character.
        ans += (left1 - left2) as i64;
    }
    
    ans
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a buffered reader for standard input.
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the first value: the size of the word (wordSize). This follows the input format.
    let word_size_line = lines.next().ok_or("Missing word size input")??;
    let word_size: usize = word_size_line.trim().parse()?;
    
    // Read the word (as a string). The C code uses scanf("%s") so we follow the same logic.
    let mut word = String::new();
    while word.trim().is_empty() {
        if let Some(line) = lines.next() {
            word = line?;
        } else {
            break;
        }
    }
    // (Optional) We could verify that word.len() == word_size, but the original C code doesn't enforce this.
    
    // Read the integer k from the input.
    let k_line = lines.next().ok_or("Missing k input")??;
    let k: i32 = k_line.trim().parse()?;
    
    // Calculate the answer using the count_of_substrings function.
    let ans = count_of_substrings(&word, k);
    
    // Print the result exactly as the C code does.
    println!("{}", ans);
    
    Ok(())
}