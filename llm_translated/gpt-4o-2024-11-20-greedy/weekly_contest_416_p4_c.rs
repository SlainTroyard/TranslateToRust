use std::io::{self, Write};
use std::collections::HashMap;

fn update(diff: &mut [i32; 26], c: usize, add: i32, cnt: &mut i32) {
    diff[c] += add;
    if add == 1 && diff[c] == 0 {
        *cnt -= 1;
    } else if add == -1 && diff[c] == -1 {
        *cnt += 1;
    }
}

fn valid_substring_count(word1: &str, word2: &str) -> i64 {
    let mut diff = [0; 26];
    for c in word2.chars() {
        diff[c as usize - 'a' as usize] -= 1;
    }

    let mut cnt = 0;
    for &value in &diff {
        if value < 0 {
            cnt += 1;
        }
    }

    let mut res = 0;
    let mut l = 0;
    let mut r = 0;
    let len1 = word1.len();
    let word1_bytes = word1.as_bytes();

    while l < len1 {
        while r < len1 && cnt > 0 {
            update(&mut diff, (word1_bytes[r] - b'a') as usize, 1, &mut cnt);
            r += 1;
        }
        if cnt == 0 {
            res += (len1 - r) as i64 + 1;
        }
        update(&mut diff, (word1_bytes[l] - b'a') as usize, -1, &mut cnt);
        l += 1;
    }

    res
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let len1: usize = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let word1 = input.trim().to_string();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let len2: usize = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let word2 = input.trim().to_string();

    let result = valid_substring_count(&word1, &word2);
    println!("{}", result);
}