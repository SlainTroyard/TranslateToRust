use std::io;

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
    for i in 0..26 {
        if diff[i] < 0 {
            cnt += 1;
        }
    }

    let mut res: i64 = 0;
    let mut l = 0;
    let mut r = 0;
    let len1 = word1.len();

    while l < len1 {
        while r < len1 && cnt > 0 {
            let char_index = (word1.chars().nth(r).unwrap() as u8 - b'a') as usize;
            update(&mut diff, char_index, 1, &mut cnt);
            r += 1;
        }
        if cnt == 0 {
            res += (len1 - r + 1) as i64;
        }
        let char_index = (word1.chars().nth(l).unwrap() as u8 - b'a') as usize;
        update(&mut diff, char_index, -1, &mut cnt);
        l += 1;
    }

    res
}

fn main() {
    let mut len1_str = String::new();
    io::stdin().read_line(&mut len1_str).expect("Failed to read line");
    let len1: usize = len1_str.trim().parse().expect("Invalid input for len1");

    let mut word1 = String::new();
    io::stdin().read_line(&mut word1).expect("Failed to read line");
    let word1 = word1.trim();

    let mut len2_str = String::new();
    io::stdin().read_line(&mut len2_str).expect("Failed to read line");
    let len2: usize = len2_str.trim().parse().expect("Invalid input for len2");

    let mut word2 = String::new();
    io::stdin().read_line(&mut word2).expect("Failed to read line");
    let word2 = word2.trim();

    println!("{}", valid_substring_count(word1, word2));
}