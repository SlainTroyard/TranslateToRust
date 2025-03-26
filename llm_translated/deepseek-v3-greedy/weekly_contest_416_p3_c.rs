use std::io::{self, Write};

fn get(l: usize, r: usize, pre_count: &Vec<[i32; 26]>, count: &[i32; 26]) -> usize {
    let mut border = l;
    let mut l = l;
    let mut r = r;
    while l < r {
        let m = (l + r) >> 1;
        let mut f = true;
        for i in 0..26 {
            if pre_count[m][i] - pre_count[border - 1][i] < count[i] {
                f = false;
                break;
            }
        }
        if f {
            r = m;
        } else {
            l = m + 1;
        }
    }
    l
}

fn valid_substring_count(word1: &str, word2: &str) -> i64 {
    let mut count = [0; 26];
    for c in word2.chars() {
        count[(c as u8 - b'a') as usize] += 1;
    }

    let n = word1.len();
    let mut pre_count = vec![[0; 26]; n + 1];
    for i in 1..=n {
        pre_count[i] = pre_count[i - 1];
        let c = word1.chars().nth(i - 1).unwrap();
        pre_count[i][(c as u8 - b'a') as usize] += 1;
    }

    let mut res = 0;
    for l in 1..=n {
        let r = get(l, n + 1, &pre_count, &count);
        res += (n - r + 1) as i64;
    }
    res
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let len1: usize = input.trim().parse().unwrap();
    
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let word1 = input.trim();
    
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let len2: usize = input.trim().parse().unwrap();
    
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let word2 = input.trim();
    
    println!("{}", valid_substring_count(word1, word2));
}