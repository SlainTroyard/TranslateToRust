use std::io::{self, BufRead, Write};

fn get(l: usize, r: usize, pre_count: &[[usize; 26]], count: &[usize]) -> usize {
    let mut left = l;
    let mut right = r;
    while left < right {
        let mid = (left + right) >> 1;
        if (0..26).all(|i| pre_count[mid][i] - pre_count[l - 1][i] >= count[i]) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    left
}

fn valid_substring_count(word1: &str, word2: &str) -> u64 {
    let mut count = [0; 26];
    for c in word2.chars() {
        count[(c as u8 - b'a') as usize] += 1;
    }

    let n = word1.len();
    let mut pre_count = vec![[0; 26]; n + 1];
    for (i, c) in word1.chars().enumerate() {
        pre_count[i + 1] = pre_count[i];
        pre_count[i + 1][(c as u8 - b'a') as usize] += 1;
    }

    let mut res = 0;
    for l in 1..=n {
        let r = get(l, n + 1, &pre_count, &count);
        res += (n - r + 1) as u64;
    }
    res
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    // Read the length of word1
    let len1: usize = stdin.lock().lines().next().unwrap()?.parse()?;
    let word1: String = stdin.lock().lines().next().unwrap()?;

    // Read the length of word2
    let len2: usize = stdin.lock().lines().next().unwrap()?.parse()?;
    let word2: String = stdin.lock().lines().next().unwrap()?;

    // Calculate and print the result
    writeln!(stdout, "{}", valid_substring_count(&word1, &word2))?;

    Ok(())
}