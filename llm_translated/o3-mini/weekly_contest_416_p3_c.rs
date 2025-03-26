use std::io::{self, Read, Write};

/// Binary search helper function, translated from C's get() function.
///
/// # Arguments
/// * `l` - starting index (1-indexed) for the search range.
/// * `r` - ending index (exclusive, 1-indexed) for the search range.
/// * `pre_count` - cumulative counts for each prefix of word1.
/// * `count` - required frequency of each letter (from word2).
///
/// Returns the smallest index `m` in the range [l, r) for which the substring
/// from index l to m (inclusive) of word1 meets or exceeds the required frequency.
fn get(l: usize, r: usize, pre_count: &Vec<[i32; 26]>, count: &[i32; 26]) -> usize {
    let border = l; // border-1 is used to get frequency counts before the current substring.
    let mut l = l;
    let mut r = r;
    while l < r {
        let m = (l + r) >> 1;
        let mut ok = true;
        // Check if the substring [border, m] contains enough occurrences of each letter.
        for i in 0..26 {
            if pre_count[m][i] - pre_count[border - 1][i] < count[i] {
                ok = false;
                break;
            }
        }
        if ok {
            r = m;
        } else {
            l = m + 1;
        }
    }
    l
}

/// Count the number of valid substrings of word1 that contain all the characters of word2
/// with at least the frequency specified in word2, keeping the same logic as the original C code.
///
/// # Arguments
/// * `word1` - the primary string (as input).
/// * `word2` - the target string whose character frequencies must be covered.
///
/// Returns the count of valid substrings as i64.
fn valid_substring_count(word1: &str, word2: &str) -> i64 {
    // Build the target character frequency count from word2.
    let mut count = [0i32; 26];
    for ch in word2.chars() {
        // Assumes letters are lowercase a-z, matching the original code.
        count[(ch as u8 - b'a') as usize] += 1;
    }

    let n = word1.len();
    // pre_count is a vector of cumulative frequency arrays.
    // We use n+1 elements because pre_count[0] is the zero vector.
    let mut pre_count = vec![[0; 26]; n + 1];
    let word1_bytes = word1.as_bytes();

    // Build cumulative frequency counts:
    // pre_count[i] stores frequencies for the first i characters (1-indexed as in C).
    for i in 1..=n {
        pre_count[i] = pre_count[i - 1];
        let index = (word1_bytes[i - 1] - b'a') as usize;
        pre_count[i][index] += 1;
    }

    let mut res: i64 = 0;
    // Loop for each starting index l (1-indexed) of word1.
    for l in 1..=n {
        // Search in the range [l, n+1) for the minimal ending index that fulfills the condition.
        let r = get(l, n + 1, &pre_count, &count);
        // If r is within the string, all substrings starting at l and ending from r to n are valid.
        res += (n - r + 1) as i64;
    }
    res
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read entire input from stdin.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    // Split input by whitespace. The original C code uses scanf with %d and %s,
    // so we assume input tokens follow that exact order.
    let mut tokens = input.split_whitespace();

    // Read the length of word1.
    let len1_token = tokens.next().ok_or("Expected len1")?;
    let len1: usize = len1_token.parse()
        .map_err(|_| "Failed to parse len1 as usize")?;

    // Read word1.
    let word1 = tokens.next().ok_or("Expected word1")?;
    // The original code does not enforce that the string length matches len1,
    // so we simply use the provided string.

    // Read the length of word2.
    let len2_token = tokens.next().ok_or("Expected len2")?;
    let len2: usize = len2_token.parse()
        .map_err(|_| "Failed to parse len2 as usize")?;

    // Read word2.
    let word2 = tokens.next().ok_or("Expected word2")?;
    // Again, we do not enforce that word2's length exactly matches len2.

    // Compute the result using valid_substring_count.
    let res = valid_substring_count(word1, word2);

    // Print the result to stdout matching the original code format.
    println!("{}", res);

    Ok(())
}