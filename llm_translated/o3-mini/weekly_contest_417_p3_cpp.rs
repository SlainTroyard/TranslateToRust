use std::io::{self, BufRead, Write};

/// This function computes the count of substrings that satisfy the problem’s condition.
/// The algorithm is implemented to exactly mimic the C++ version provided.
///
/// Variables:
/// - VOWEL_MASK: A constant bitmask used to quickly check if a character is a vowel.
///   In the mask, the bits corresponding to 'a', 'e', 'i', 'o', 'u' (positions 0, 4, 8, 14, 20)
///   are set. The constant value 1065233 represents that mask.
/// - cnt_vowel1, cnt_vowel2: Arrays (of size 26) to maintain frequency counts for vowels in two sliding windows.
/// - size_vowel1, size_vowel2: Counters for the number of distinct vowels in each window.
/// - cnt_consonant1, cnt_consonant2: Counters for the number of consonants in each window.
/// - left1, left2: Two left pointers corresponding to the two sliding windows.
fn count_of_substrings(word: &str, k: i32) -> i64 {
    const VOWEL_MASK: u32 = 1065233;
    let mut ans: i64 = 0;

    // Create frequency arrays for vowels (we use size 26 for simplicity)
    let mut cnt_vowel1 = [0; 26];
    let mut cnt_vowel2 = [0; 26];
    let mut size_vowel1 = 0;
    let mut size_vowel2 = 0;
    let mut cnt_consonant1 = 0;
    let mut cnt_consonant2 = 0;
    let mut left1 = 0;
    let mut left2 = 0;

    // Convert the word into a byte array for O(1) indexing and arithmetic
    let word_bytes = word.as_bytes();

    // Iterate over each character index as the right-bound of the sliding window
    for right in 0..word_bytes.len() {
        // Convert the byte to an index (0 for 'a', 1 for 'b', etc.)
        let b = (word_bytes[right] - b'a') as usize;
        // Check if the character is a vowel using VOWEL_MASK.
        if ((VOWEL_MASK >> b) & 1) != 0 {
            if cnt_vowel1[b] == 0 {
                size_vowel1 += 1;
            }
            cnt_vowel1[b] += 1;
            if cnt_vowel2[b] == 0 {
                size_vowel2 += 1;
            }
            cnt_vowel2[b] += 1;
        } else {
            cnt_consonant1 += 1;
            cnt_consonant2 += 1;
        }

        // Adjust the first window while the condition holds:
        // the window has all 5 vowels and at least k consonants.
        while size_vowel1 == 5 && cnt_consonant1 >= k {
            let out = (word_bytes[left1] - b'a') as usize;
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

        // Adjust the second window with a slightly different consonant condition: > k
        while size_vowel2 == 5 && cnt_consonant2 > k {
            let out = (word_bytes[left2] - b'a') as usize;
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

        // The difference between left pointers gives the contribution for the current right index.
        ans += (left1 - left2) as i64;
    }
    ans
}

fn main() -> io::Result<()> {
    // Set up buffered input and output for efficiency.
    let stdin = io::stdin();
    let mut reader = io::BufReader::new(stdin.lock());
    let stdout = io::stdout();
    let mut writer = io::BufWriter::new(stdout.lock());

    // Read the entire input into a string.
    let mut input = String::new();
    reader.read_to_string(&mut input)?;

    // Split the input by whitespace.
    let mut tokens = input.split_whitespace();

    // Parse the input exactly as in the C++ code:
    // - First integer: word size (although not used in logic, we read it for consistency)
    // - Then the actual word
    // - Then integer k
    let word_size: usize = tokens
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Missing word size"))?
        .parse()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid word size"))?;
    let word = tokens
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Missing word"))?;
    // Ensure the length of the word matches word_size (if needed, as in C++ the size is ignored)
    if word.len() != word_size {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Word size does not match the provided word",
        ));
    }
    let k: i32 = tokens
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Missing k"))?
        .parse()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid k"))?;

    // Compute the answer using the provided algorithm.
    let ans = count_of_substrings(word, k);

    // Write the result to stdout matching the exact format: just the answer.
    writeln!(writer, "{}", ans)?;
    Ok(())
}