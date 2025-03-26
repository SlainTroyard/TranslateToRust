use std::collections::HashMap;
use std::io;

fn max_substring_length(s: String, k: i32) -> bool {
    let n = s.len();
    let mut pos: Vec<Vec<usize>> = vec![vec![]; 26];

    for (i, c) in s.chars().enumerate() {
        let idx = (c as u8 - b'a') as usize;
        pos[idx].push(i);
    }

    let mut vec: Vec<(usize, usize)> = Vec::new();
    for i in 0..26 {
        if !pos[i].is_empty() {
            let mut l = pos[i][0];
            let mut r = pos[i][pos[i].len() - 1];
            let mut flag = true;
            while flag {
                flag = false;
                for j in 0..26 {
                    let cnt = pos[j].partition_point(|&x| x <= r) - pos[j].partition_point(|&x| x < l);
                    if cnt > 0 && cnt < pos[j].len() {
                        l = l.min(pos[j][0]);
                        r = r.max(pos[j][pos[j].len() - 1]);
                        flag = true;
                    }
                }
            }
            if l > 0 || r < n - 1 {
                vec.push((r, l));
            }
        }
    }

    vec.sort();
    let mut r = 0;
    let mut cnt = 0;
    for p in vec {
        if p.1 > r {
            r = p.0;
            cnt += 1;
        }
    }
    cnt >= k
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let s = iter.next().expect("No string provided").to_string();
    let k: i32 = iter.next().expect("No K provided").parse().expect("K should be a number");

    println!("{}", max_substring_length(s, k));
}