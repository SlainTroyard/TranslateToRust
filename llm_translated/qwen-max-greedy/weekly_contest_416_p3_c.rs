use std::io::{self, BufRead, Write};

fn get(l: usize, r: usize, pre_count: &[[usize; 26]], count: &[usize]) -> usize {
    let mut border = l;
    while l < r {
        let m = (l + r) >> 1;
        let mut f = 1;
        for i in 0..26 {
            if pre_count[m][i] - pre_count[border - 1][i] < count[i] {
                f = 0;
                break;
            }
        }
        if f != 0 {
            r = m;
        } else {
            l = m + 1;
        }
    }
    l
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
    buffer.clear();

    // Calculate and print the result
    writeln!(stdout, "{}", valid_substring_count(word1, word2)).unwrap();
}