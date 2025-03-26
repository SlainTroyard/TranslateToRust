use std::io::{self, BufRead};

/// A simple tokenizer for standard input, replicating scanf("%d") / scanf("%s") logic.
/// It reads and splits input by whitespace, providing one token at a time.
struct Scanner {
    tokens: Vec<String>,
    idx: usize,
}

impl Scanner {
    fn new() -> Self {
        Scanner {
            tokens: Vec::new(),
            idx: 0,
        }
    }

    /// Read the next token from cin, auto-refilling tokens if necessary.
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            // If we've used up all tokens, read the next line
            if self.idx >= self.tokens.len() {
                let mut buf = String::new();
                io::stdin().lock().read_line(&mut buf).expect("Failed to read input");
                // Skip empty lines
                if buf.trim().is_empty() {
                    continue;
                }
                self.tokens = buf.split_whitespace().map(|s| s.to_string()).collect();
                self.idx = 0;
            }
            // Attempt to parse the current token
            if let Ok(value) = self.tokens[self.idx].parse::<T>() {
                self.idx += 1;
                return value;
            } else {
                panic!("Failed to parse token as the requested type");
            }
        }
    }
}

/// Updates the diff array and cnt value, replicating the C 'update' function exactly.
fn update(diff: &mut [i32; 26], c: usize, add: i32, cnt: &mut i32) {
    diff[c] += add;
    if add == 1 && diff[c] == 0 {
        *cnt -= 1;
    } else if add == -1 && diff[c] == -1 {
        *cnt += 1;
    }
}

/// Counts the number of valid substrings in word1 given the requirements involving word2.
/// Translated directly from the C 'validSubstringCount' function.
fn valid_substring_count(word1: &str, word2: &str) -> i64 {
    // Initialize diff array
    let mut diff = [0i32; 26];
    // Decrement for each character in word2
    for c in word2.chars() {
        let idx = (c as u8 - b'a') as usize;
        diff[idx] -= 1;
    }

    // Count how many are negative
    let mut cnt = 0i32;
    for &val in &diff {
        if val < 0 {
            cnt += 1;
        }
    }

    let mut res = 0i64;
    let (mut l, mut r) = (0usize, 0usize);
    let len1 = word1.len();
    let bytes = word1.as_bytes();

    // Two-pointer approach
    while l < len1 {
        // Expand the right pointer until all differences are satisfied
        while r < len1 && cnt > 0 {
            let c = (bytes[r] - b'a') as usize;
            update(&mut diff, c, 1, &mut cnt);
            r += 1;
        }
        // If cnt == 0, all requirements are met for substrings starting at l
        if cnt == 0 {
            res += (len1 - r) as i64 + 1;
        }
        // Move left pointer (shrink the window)
        let c = (bytes[l] - b'a') as usize;
        update(&mut diff, c, -1, &mut cnt);
        l += 1;
    }

    res
}

fn main() {
    let mut scanner = Scanner::new();

    // Read input exactly as in the C code
    let len1: usize = scanner.next(); // corresponds to scanf("%d", &len1)
    let word1: String = scanner.next(); // corresponds to scanf("%s", word1)
    let len2: usize = scanner.next(); // corresponds to scanf("%d", &len2)
    let word2: String = scanner.next(); // corresponds to scanf("%s", word2)

    // Compute and print result
    let answer = valid_substring_count(&word1, &word2);
    println!("{}", answer);
}