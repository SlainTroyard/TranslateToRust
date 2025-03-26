use std::io::{self, Read};

type PrefixCount = [i32; 26];

fn get(l: usize, r: usize, pre_count: &[PrefixCount], count: &[i32; 26]) -> usize {
    let mut left = l;
    let mut right = r;
    let border = l;

    while left < right {
        let mid = (left + right) / 2;
        let mut valid = true;
        for i in 0..26 {
            if pre_count[mid][i] - pre_count[border - 1][i] < count[i] {
                valid = false;
                break;
            }
        }
        if valid {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    left
}

fn valid_substring_count(word1: &str, word2: &str) -> i64 {
    let mut count = [0; 26];
    for c in word2.bytes() {
        count[(c - b'a') as usize] += 1;
    }

    let n = word1.len();
    let mut pre_count = vec![[0; 26]; n + 1];
    for i in 1..=n {
        pre_count[i] = pre_count[i - 1];
        let c = word1.as_bytes()[i - 1] - b'a';
        pre_count[i][c as usize] += 1;
    }

    let mut res = 0;
    for l in 1..=n {
        let r = get(l, n + 1, &pre_count, &count);
        res += (n as i64) - (r as i64) + 1;
    }
    res
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut tokens = input.split_whitespace();

    let len1: usize = tokens
        .next()
        .expect("Missing len1")
        .parse()
        .expect("Invalid len1");
    let word1 = tokens.next().expect("Missing word1");
    let len2: usize = tokens
        .next()
        .expect("Missing len2")
        .parse()
        .expect("Invalid len2");
    let word2 = tokens.next().expect("Missing word2");

    println!("{}", valid_substring_count(word1, word2));
}