// LeetCode Weekly Contest 416 Problem 3 in Rust
//
// This program reads two lengths (len1, len2) followed by two strings (word1, word2)
// from standard input, then computes and prints the number of valid substrings
// in 'word1' that contain all characters of 'word2' at least as many times as they
// appear in 'word2'. The output is a single integer on its own line.
//
// The I/O format is identical to the original C code:
//
// Input:
//   <len1>
//   <word1>
//   <len2>
//   <word2>
//
// Output:
//   <valid_substring_count>
//
// Example:
//   4
//   abab
//   2
//   ab
//
//   Output:
//   7

use std::io::{self, BufRead};

/// Binary-search helper, replicating the 'get' function from the C code.
///
/// Searches within [l, r) for the smallest m such that
/// for all characters i, pre_count[m][i] - pre_count[l-1][i] >= count[i].
fn get(
    mut l: usize,
    mut r: usize,
    pre_count: &[ [usize; 26] ],
    count: &[usize; 26],
) -> usize {
    let border = l;
    // Standard binary search for the smallest r that covers all needed characters
    while l < r {
        let m = (l + r) >> 1;
        let mut found = true;
        for i in 0..26 {
            if pre_count[m][i] < pre_count[border - 1][i] + count[i] {
                found = false;
                break;
            }
        }
        if found {
            r = m;
        } else {
            l = m + 1;
        }
    }
    l
}

/// Counts the number of valid substrings of 'word1' that contain all characters
/// in 'word2' at least as many times as they appear in 'word2'.
fn valid_substring_count(word1: &str, word2: &str) -> i64 {
    let mut count = [0_usize; 26];
    for &b in word2.as_bytes() {
        count[(b - b'a') as usize] += 1;
    }

    let n = word1.len();
    // Build prefix count array (1-based indexing: pre_count[i] is count up to i)
    let mut pre_count = vec![[0_usize; 26]; n + 1];
    for (i, &b) in word1.as_bytes().iter().enumerate() {
        pre_count[i + 1] = pre_count[i]; // copy previous
        pre_count[i + 1][(b - b'a') as usize] += 1;
    }

    let mut res = 0_i64;
    // For each starting position l = 1..n (inclusive), find the earliest r
    // such that substring [l, r] contains all needed characters.
    for l in 1..=n {
        let r = get(l, n + 1, &pre_count, &count);
        // If r is within [l, n], number of valid end positions for substring
        // starting at l is all positions from r..n. So add (n - r + 1).
        res += (n - r + 1) as i64;
    }
    res
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read len1
    let len1: usize = lines
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "Missing len1"))??
        .trim()
        .parse()
        .expect("Failed to parse len1");

    // Read word1
    let word1: String = lines
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "Missing word1"))??
        .trim()
        .to_string();
    assert_eq!(word1.len(), len1, "word1 length does not match len1");

    // Read len2
    let len2: usize = lines
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "Missing len2"))??
        .trim()
        .parse()
        .expect("Failed to parse len2");

    // Read word2
    let word2: String = lines
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "Missing word2"))??
        .trim()
        .to_string();
    assert_eq!(word2.len(), len2, "word2 length does not match len2");

    // Compute and print result
    let result = valid_substring_count(&word1, &word2);
    println!("{}", result);

    Ok(())
}