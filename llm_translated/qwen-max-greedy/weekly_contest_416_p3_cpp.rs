use std::io::{self, BufRead};

fn valid_substring_count(word1: &str, word2: &str) -> i64 {
    let mut count = vec![0; 26];
    for c in word2.chars() {
        count[(c as u8 - b'a') as usize] += 1;
    }

    let n = word1.len();
    let mut pre_count = vec![vec![0; 26]; n + 1];
    for (i, c) in word1.chars().enumerate() {
        pre_count[i + 1] = pre_count[i].clone();
        pre_count[i + 1][(c as u8 - b'a') as usize] += 1;
    }

    let get = |l: usize, r: usize| -> usize {
        let mut l = l;
        let mut r = r;
        while l < r {
            let m = l + (r - l) / 2;
            if (0..26).all(|i| pre_count[m][i] - pre_count[l - 1][i] >= count[i]) {
                r = m;
            } else {
                l = m + 1;
            }
        }
        l
    };

    let mut res = 0;
    for l in 1..=n {
        let r = get(l, n + 1);
        res += (n - r + 1) as i64;
    }
    res
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first line to get the length of word1
    let len1: usize = lines.next().unwrap().unwrap().trim().parse().expect("Failed to parse len1");
    let word1 = lines.next().unwrap().unwrap();

    // Read the second line to get the length of word2
    let len2: usize = lines.next().unwrap().unwrap().trim().parse().expect("Failed to parse len2");
    let word2 = lines.next().unwrap().unwrap();

    let result = valid_substring_count(&word1, &word2);
    println!("{}", result);
}