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
                    let cnt = pos[j].upper_bound(r) - pos[j].lower_bound(l);
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
    for &(end, start) in &vec {
        if start > r {
            r = end;
            cnt += 1;
        }
    }
    cnt >= k
}

trait Bound {
    fn lower_bound(&self, x: usize) -> usize;
    fn upper_bound(&self, x: usize) -> usize;
}

impl Bound for Vec<usize> {
    fn lower_bound(&self, x: usize) -> usize {
        self.binary_search(&x).unwrap_or_else(|i| i)
    }

    fn upper_bound(&self, x: usize) -> usize {
        match self.binary_search(&x) {
            Ok(i) => i + 1,
            Err(i) => i,
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let s = input.trim().to_string();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let k: i32 = input.trim().parse().expect("Failed to parse K");

    let result = max_substring_length(s, k);
    println!("{}", result);
}