use std::io::{self, Read};
use std::cmp::Ordering;

fn get(l: usize, r: usize, pre_count: &[Vec<usize>], count: &[usize]) -> usize {
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
    for &b in word2.as_bytes() {
        count[(b - b'a') as usize] += 1;
    }

    let n = word1.len();
    let mut pre_count = vec![vec![0; 26]; n + 1];

    for (i, &b) in word1.as_bytes().iter().enumerate() {
        pre_count[i + 1] = pre_count[i].clone();
        pre_count[i + 1][(b - b'a') as usize] += 1;
    }

    let mut res = 0i64;
    for l in 1..=n {
        let r = get(l, n + 1, &pre_count, &count);
        res += (n - r + 1) as i64;
    }

    res
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    // Read the length of word1
    let len1: usize = lines.next().unwrap().parse().unwrap();
    // Read word1
    let word1 = lines.next().unwrap();
    assert_eq!(word1.len(), len1);

    // Read the length of word2
    let len2: usize = lines.next().unwrap().parse().unwrap();
    // Read word2
    let word2 = lines.next().unwrap();
    assert_eq!(word2.len(), len2);

    // Compute the result
    let result = valid_substring_count(word1, word2);

    // Print the result
    println!("{}", result);
}