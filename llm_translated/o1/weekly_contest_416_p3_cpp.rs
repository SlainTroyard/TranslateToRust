// Translated from C++ to Rust for LeetCode Weekly Contest 416 Problem 3.
// The program reads four lines of input:
// 1) An integer len1
// 2) A string word1
// 3) An integer len2
// 4) A string word2
// It then prints the result of validSubstringCount(word1, word2) to stdout.

use std::error::Error;
use std::io::{self, BufRead};

/// Returns the count of valid substrings of `word1` that contain at least 
/// the character frequencies present in `word2`.
fn valid_substring_count(word1: &str, word2: &str) -> i64 {
    // Count frequency of each character in word2
    let mut count = vec![0; 26];
    for c in word2.chars() {
        count[c as usize - 'a' as usize] += 1;
    }

    let n = word1.len();
    // pre_count[i] will store the frequencies of each character from 
    // the start of word1 up to (and including) the (i-1)-th character of word1.
    let mut pre_count = vec![vec![0; 26]; n + 1];
    for i in 1..=n {
        pre_count[i] = pre_count[i - 1].clone();
        let idx = word1.as_bytes()[i - 1] as usize - b'a' as usize;
        pre_count[i][idx] += 1;
    }

    // Function to find the smallest index m in [l, r) such that
    // the substring word1[l..=m] contains at least the frequencies in `count`.
    let get = |mut l: usize, mut r: usize| {
        let border = l;
        while l < r {
            // Midpoint
            let m = (l + r) >> 1;
            // Check if substring word1[border..=m] satisfies frequency requirement
            let mut valid = true;
            for i in 0..26 {
                if pre_count[m][i] - pre_count[border - 1][i] < count[i] {
                    valid = false;
                    break;
                }
            }
            if valid {
                r = m;
            } else {
                l = m + 1;
            }
        }
        l
    };

    // Sum over all valid substrings
    let mut res = 0i64;
    for l in 1..=n {
        let r = get(l, n + 1);
        res += (n - r + 1) as i64;
    }
    res
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read len1
    let len1 = lines
        .next()
        .ok_or("Missing input for len1")??
        .trim()
        .parse::<usize>()?;
    // Read word1
    let word1 = lines
        .next()
        .ok_or("Missing input for word1")??
        .trim()
        .to_string();
    // Read len2
    let len2 = lines
        .next()
        .ok_or("Missing input for len2")??
        .trim()
        .parse::<usize>()?;
    // Read word2
    let word2 = lines
        .next()
        .ok_or("Missing input for word2")??
        .trim()
        .to_string();

    // Call our function and print result
    let result = valid_substring_count(&word1, &word2);
    println!("{}", result);

    Ok(())
}