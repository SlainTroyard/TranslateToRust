use std::io::{self, BufRead, Write};

// Helper function to update the difference array and count of missing characters
fn update(diff: &mut [i32; 26], c: usize, add: i32, cnt: &mut i32) {
    diff[c] += add;
    if add == 1 && diff[c] == 0 {
        *cnt -= 1;
    } else if add == -1 && diff[c] == -1 {
        *cnt += 1;
    }
}

// Function to calculate the number of valid substrings
fn valid_substring_count(word1: &str, word2: &str) -> i64 {
    let mut diff = [0; 26];
    for c in word2.chars() {
        diff[(c as u8 - b'a') as usize] -= 1;
    }

    let mut cnt = 0;
    for &d in diff.iter() {
        if d < 0 {
            cnt += 1;
        }
    }

    let mut res = 0;
    let (mut l, mut r) = (0, 0);
    let len1 = word1.len();
    while l < len1 {
        while r < len1 && cnt > 0 {
            update(&mut diff, (word1.as_bytes()[r] - b'a') as usize, 1, &mut cnt);
            r += 1;
        }
        if cnt == 0 {
            res += (len1 - r + 1) as i64;
        }
        update(&mut diff, (word1.as_bytes()[l] - b'a') as usize, -1, &mut cnt);
        l += 1;
    }

    res
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer = String::new();

    // Read the length of the first word
    stdin.lock().read_line(&mut buffer).unwrap();
    let len1: usize = buffer.trim().parse().unwrap();
    buffer.clear();

    // Read the first word
    stdin.lock().read_line(&mut buffer).unwrap();
    let word1 = buffer.trim();
    buffer.clear();

    // Read the length of the second word
    stdin.lock().read_line(&mut buffer).unwrap();
    let len2: usize = buffer.trim().parse().unwrap();
    buffer.clear();

    // Read the second word
    stdin.lock().read_line(&mut buffer).unwrap();
    let word2 = buffer.trim();

    // Calculate the result and print it
    let result = valid_substring_count(word1, word2);
    writeln!(stdout, "{}", result).unwrap();
}