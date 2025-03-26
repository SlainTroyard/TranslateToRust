use std::io::{self, Write};

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
        diff[(c as u8 - b'a') as usize] -= 1;
    }

    let mut cnt = 0;
    for &d in &diff {
        if d < 0 {
            cnt += 1;
        }
    }

    let mut res = 0;
    let len1 = word1.len();
    let word1_bytes = word1.as_bytes();
    let mut l = 0;
    let mut r = 0;

    while l < len1 {
        while r < len1 && cnt > 0 {
            update(&mut diff, (word1_bytes[r] - b'a') as usize, 1, &mut cnt);
            r += 1;
        }
        if cnt == 0 {
            res += (len1 - r + 1) as i64;
        }
        update(&mut diff, (word1_bytes[l] - b'a') as usize, -1, &mut cnt);
        l += 1;
    }

    res
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let len1: usize = input.trim().parse().expect("Invalid input");

    let mut word1 = String::new();
    io::stdin().read_line(&mut word1).expect("Failed to read line");
    word1 = word1.trim().to_string();

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let len2: usize = input.trim().parse().expect("Invalid input");

    let mut word2 = String::new();
    io::stdin().read_line(&mut word2).expect("Failed to read line");
    word2 = word2.trim().to_string();

    let result = valid_substring_count(&word1, &word2);
    println!("{}", result);
}