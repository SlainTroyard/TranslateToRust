use std::io::{self, Read};

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

    let mut res = 0;
    let mut l = 0;
    let mut r = 0;
    let len1 = word1.len();
    while l < len1 {
        while r < len1 && cnt > 0 {
            update(&mut diff, (word1.chars().nth(r).unwrap() as u8 - b'a') as usize, 1, &mut cnt);
            r += 1;
        }
        if cnt == 0 {
            res += (len1 - r + 1) as i64;
        }
        update(&mut diff, (word1.chars().nth(l).unwrap() as u8 - b'a') as usize, -1, &mut cnt);
        l += 1;
    }

    res
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let len1: usize = lines.next().unwrap().parse().unwrap();
    let word1 = lines.next().unwrap().to_string();
    let len2: usize = lines.next().unwrap().parse().unwrap();
    let word2 = lines.next().unwrap().to_string();

    let result = valid_substring_count(&word1, &word2);
    println!("{}", result);

    Ok(())
}