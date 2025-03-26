use std::collections::HashMap;
use std::io;

fn main() {
    // Read the length of word1
    let mut len1 = String::new();
    io::stdin().read_line(&mut len1).expect("Failed to read line");
    let len1: usize = len1.trim().parse().expect("Please type a number!");

    // Read word1
    let mut word1 = String::new();
    io::stdin().read_line(&mut word1).expect("Failed to read line");
    let word1 = word1.trim().to_string();

    // Read the length of word2
    let mut len2 = String::new();
    io::stdin().read_line(&mut len2).expect("Failed to read line");
    let len2: usize = len2.trim().parse().expect("Please type a number!");

    // Read word2
    let mut word2 = String::new();
    io::stdin().read_line(&mut word2).expect("Failed to read line");
    let word2 = word2.trim().to_string();

    // Calculate and print the result
    let result = valid_substring_count(word1, word2);
    println!("{}", result);
}

fn valid_substring_count(word1: String, word2: String) -> i64 {
    let mut diff = [0; 26];
    for c in word2.chars() {
        diff[(c as u8 - b'a') as usize] -= 1;
    }

    let mut res = 0;
    let mut cnt = diff.iter().filter(|&&c| c < 0).count();

    let mut update = |c: usize, add: i32| {
        diff[c] += add;
        if add == 1 && diff[c] == 0 {
            cnt -= 1;
        } else if add == -1 && diff[c] == -1 {
            cnt += 1;
        }
    };

    let mut r = 0;
    for l in 0..word1.len() {
        while r < word1.len() && cnt > 0 {
            update((word1.chars().nth(r).unwrap() as u8 - b'a') as usize, 1);
            r += 1;
        }
        if cnt == 0 {
            res += (word1.len() - r + 1) as i64;
        }
        update((word1.chars().nth(l).unwrap() as u8 - b'a') as usize, -1);
    }

    res
}