use std::io::{self, BufRead};

fn valid_substring_count(word1: &str, word2: &str) -> i64 {
    // Create letter count array for word2 (26 letters)
    let mut count = [0; 26];
    for c in word2.chars() {
        count[(c as u8 - b'a') as usize] += 1;
    }

    let n = word1.len();
    // Create prefix count vector with n+1 elements. pre_count[0] is all zeros.
    // We'll use a Vec<[i32; 26]> to store counts for each prefix.
    let mut pre_count = vec![[0; 26]; n + 1];

    // Build the prefix counts for word1; using 1-indexed logic.
    for (i, c) in word1.chars().enumerate() {
        let mut current = pre_count[i]; // copy previous prefix count
        current[(c as u8 - b'a') as usize] += 1;
        pre_count[i + 1] = current;
    }

    // Define the binary search function using a closure.
    // Given left index "l" (1-indexed) and right index "r" (exclusive upper bound, n+1),
    // find the minimum index m in [l, r) such that for every character, 
    // count in the substring word1[l-1..m] is at least as much as required in word2.
    let get = |mut l: usize, mut r: usize| -> usize {
        let border = l; // original left border used for index offset in pre_count
        while l < r {
            // Compute mid as (l + r) / 2
            let m = (l + r) / 2;
            let mut valid = true;
            // Check if for every character, the substring count is sufficient.
            for i in 0..26 {
                // pre_count[m][i] - pre_count[border - 1][i] gives the count in substring word1[border-1..m]
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

    let mut res: i64 = 0;
    // Iterate over all possible starting positions (1-indexed)
    for l in 1..=n {
        // r is the minimal end index where substring [l, r] is valid.
        let r = get(l, n + 1);
        // All substrings starting at l with end from r to n are valid.
        res += (n - r + 1) as i64;
    }
    res
}

fn main() -> io::Result<()> {
    // Set up input reader
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first integer: len1 (but we don't actually need the length of word1 in our logic)
    let len1_line = lines.next().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Missing len1"))??;
    let _len1: usize = len1_line.trim().parse().map_err(|e| {
        io::Error::new(io::ErrorKind::InvalidInput, format!("Invalid len1: {}", e))
    })?;

    // Read word1
    let word1 = lines.next().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Missing word1"))??;
    let word1 = word1.trim(); // remove leading/trailing spaces

    // Read the second integer: len2 (we don't actually use it beyond input consistency)
    let len2_line = lines.next().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Missing len2"))??;
    let _len2: usize = len2_line.trim().parse().map_err(|e| {
        io::Error::new(io::ErrorKind::InvalidInput, format!("Invalid len2: {}", e))
    })?;

    // Read word2
    let word2 = lines.next().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Missing word2"))??;
    let word2 = word2.trim();

    // Compute the number of valid substrings
    let result = valid_substring_count(word1, word2);

    // Print the result to stdout
    println!("{}", result);

    Ok(())
}