use std::cmp;
use std::error::Error;
use std::io::{self, BufRead};

/// Helper function to find the first index in `slice` where the element is not less than `value`.
fn lower_bound(slice: &[usize], value: usize) -> usize {
    let mut low = 0;
    let mut high = slice.len();
    while low < high {
        let mid = (low + high) / 2;
        if slice[mid] < value {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    low
}

/// Helper function to find the first index in `slice` where the element is greater than `value`.
fn upper_bound(slice: &[usize], value: usize) -> usize {
    let mut low = 0;
    let mut high = slice.len();
    while low < high {
        let mid = (low + high) / 2;
        if slice[mid] <= value {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    low
}

/// The Solution struct encapsulates the solution method.
struct Solution;

impl Solution {
    /// Checks if there exists a collection of substrings (as defined by the algorithm) with count >= k.
    ///
    /// The function mimics the algorithm from the given C++ code.
    pub fn max_substring_length(s: &str, k: usize) -> bool {
        // Determine the length of the input string.
        let n = s.len();

        // Initialize 26 vectors for positions of each character 'a' to 'z'.
        let mut pos: Vec<Vec<usize>> = vec![Vec::new(); 26];

        // Fill the pos vector with indices for each character.
        // Here, we assume that the characters in s are lowercase English letters.
        for (i, ch) in s.char_indices() {
            let index = (ch as u8 - b'a') as usize;
            pos[index].push(i);
        }

        // Vector to hold pairs (r, l) representing the substrings boundaries.
        let mut vec_pairs: Vec<(usize, usize)> = Vec::new();

        // For every character that appears, compute the extreme positions and then try to merge ranges.
        for i in 0..26 {
            if !pos[i].is_empty() {
                // l is the first occurrence, r is the last occurrence of the current character.
                let mut l = pos[i][0];
                let mut r = *pos[i].last().unwrap();
                let mut flag = true;
                // Merge ranges as long as there exists a character whose occurrence count between l and r 
                // is positive but not the entire set of its occurrences.
                while flag {
                    flag = false;
                    for j in 0..26 {
                        if pos[j].is_empty() {
                            continue;
                        }
                        // Use binary search to count occurrences of the j-th character in the range [l, r].
                        let lb = lower_bound(&pos[j], l);
                        let ub = upper_bound(&pos[j], r);
                        let cnt = ub - lb;
                        if cnt > 0 && cnt < pos[j].len() {
                            // Update l and r to merge with the outer bounds of this character’s occurrences.
                            l = cmp::min(l, pos[j][0]);
                            r = cmp::max(r, *pos[j].last().unwrap());
                            flag = true;
                        }
                    }
                }
                // If the merged substring doesn't cover the entire string s then add it to the vector.
                if l > 0 || r < n - 1 {
                    vec_pairs.push((r, l));
                }
            }
        }

        // Sort the pairs by the ending index r (and then l if necessary).
        vec_pairs.sort();

        // Greedily count the maximum number of non-overlapping substrings.
        let mut R: isize = -1;
        let mut cnt = 0;
        for (r, l) in vec_pairs {
            if (l as isize) > R {
                R = r as isize;
                cnt += 1;
            }
        }
        cnt >= k
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Create an input handle and collect all tokens from the input.
    let stdin = io::stdin();
    let mut tokens: Vec<String> = Vec::new();
    for line in stdin.lock().lines() {
        let line = line?; // Propagate any I/O errors.
        // Split the line by whitespace and collect all tokens.
        for token in line.split_whitespace() {
            tokens.push(token.to_string());
        }
    }

    // Check that there are at least two tokens (string s and integer k).
    if tokens.len() < 2 {
        return Err("Not enough input.".into());
    }
    // The first token is the string; the second token is the integer k.
    let s = tokens[0].clone();
    let k: usize = tokens[1].parse()?;

    // Use the solution function to compute the result.
    let result = Solution::max_substring_length(&s, k);

    // Output as 1 for true and 0 for false to match the original C++ behavior.
    println!("{}", if result { 1 } else { 0 });

    Ok(())
}