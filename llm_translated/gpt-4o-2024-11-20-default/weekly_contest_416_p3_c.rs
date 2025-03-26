use std::io::{self, BufRead};
use std::cmp::Ordering;

fn get(l: usize, r: usize, pre_count: &[Vec<i32>], count: &[i32]) -> usize {
    let mut left = l;
    let mut right = r;

    while left < right {
        let m = (left + right) / 2;
        let mut valid = true;
        for i in 0..26 {
            if pre_count[m][i] - pre_count[l - 1][i] < count[i] {
                valid = false;
                break;
            }
        }
        if valid {
            right = m;
        } else {
            left = m + 1;
        }
    }
    left
}

fn valid_substring_count(word1: &str, word2: &str) -> i64 {
    let mut count = vec![0; 26];
    for c in word2.chars() {
        count[(c as usize) - ('a' as usize)] += 1;
    }

    let n = word1.len();
    let mut pre_count = vec![vec![0; 26]; n + 1];
    for (i, c) in word1.chars().enumerate() {
        pre_count[i + 1] = pre_count[i].clone();
        pre_count[i + 1][(c as usize) - ('a' as usize)] += 1;
    }

    let mut res = 0;
    for l in 1..=n {
        let r = get(l, n + 1, &pre_count, &count);
        res += (n - r + 1) as i64;
    }

    res
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the length and the first word
    let len1: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let word1: String = lines.next().unwrap().unwrap();

    // Read the length and the second word
    let len2: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let word2: String = lines.next().unwrap().unwrap();

    // Ensure inputs are valid
    assert_eq!(len1, word1.len());
    assert_eq!(len2, word2.len());

    // Calculate and print the result
    let result = valid_substring_count(&word1, &word2);
    println!("{}", result);
}