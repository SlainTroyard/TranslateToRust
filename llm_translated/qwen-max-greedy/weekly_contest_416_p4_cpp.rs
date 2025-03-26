use std::io::{self, BufRead, Write};

fn valid_substring_count(word1: &str, word2: &str) -> i64 {
    let mut diff = vec![0; 26];
    for c in word2.chars() {
        diff[c as usize - 'a' as usize] -= 1;
    }

    let mut res = 0;
    let mut cnt = diff.iter().filter(|&&c| c < 0).count();
    let update = |c: usize, add: isize| {
        diff[c] += add;
        if add == 1 && diff[c] == 0 {
            // Indicates that diff[c] changed from -1 to 0
            cnt -= 1;
        } else if add == -1 && diff[c] == -1 {
            // Indicates that diff[c] changed from 0 to -1
            cnt += 1;
        }
    };

    let mut r = 0;
    for (l, c) in word1.chars().enumerate() {
        while r < word1.len() && cnt > 0 {
            if let Some(next_c) = word1.chars().nth(r) {
                update(next_c as usize - 'a' as usize, 1);
            }
            r += 1;
        }
        if cnt == 0 {
            res += (word1.len() - r + 1) as i64;
        }
        update(c as usize - 'a' as usize, -1);
    }
    res
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer = String::new();

    // Read the length of word1
    stdin.lock().read_line(&mut buffer).unwrap();
    let len1: usize = buffer.trim().parse().unwrap();
    buffer.clear();

    // Read word1
    stdin.lock().read_line(&mut buffer).unwrap();
    let word1 = buffer.trim();
    buffer.clear();

    // Read the length of word2
    stdin.lock().read_line(&mut buffer).unwrap();
    let len2: usize = buffer.trim().parse().unwrap();
    buffer.clear();

    // Read word2
    stdin.lock().read_line(&mut buffer).unwrap();
    let word2 = buffer.trim();
    buffer.clear();

    // Create an instance of the Solution and compute the result
    let result = valid_substring_count(word1, word2);

    // Output the result
    writeln!(stdout, "{}", result).unwrap();
}