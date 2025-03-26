use std::io;
use std::io::BufRead;
use std::cmp::{min, max};

fn solve() -> bool {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let s = lines.next().unwrap().expect("Failed to read line");
    let k_str = lines.next().unwrap().expect("Failed to read line");
    let k: i32 = k_str.trim().parse().expect("Failed to parse K as i32");

    let n = s.len();
    let mut pos: Vec<Vec<usize>> = vec![Vec::new(); 26];
    for (i, char) in s.chars().enumerate() {
        let c = (char as u8 - b'a') as usize;
        pos[c].push(i);
    }

    let mut vec: Vec<(usize, usize)> = Vec::new();
    for i in 0..26 {
        if !pos[i].is_empty() {
            let mut l = pos[i][0];
            let mut r = *pos[i].last().unwrap();
            let mut flag = true;
            while flag {
                flag = false;
                for j in 0..26 {
                    let count = {
                        let start = pos[j].binary_search(&l).unwrap_or_else(|x| x);
                        let end = pos[j].binary_search(&r).unwrap_or_else(|x| x);
                        end - start
                    };
                    if count > 0 && count < pos[j].len() {
                        l = min(l, pos[j][0]);
                        r = max(r, *pos[j].last().unwrap());
                        flag = true;
                    }
                }
            }
            if l > 0 || r < n - 1 {
                vec.push((r, l));
            }
        }
    }

    vec.sort_by_key(|p| p.0);
    let mut r_max = -1_isize;
    let mut count = 0;
    for p in vec {
        if p.1 as isize > r_max {
            r_max = p.0 as isize;
            count += 1;
        }
    }
    count >= k as usize
}

fn main() {
    if solve() {
        println!("1");
    } else {
        println!("0");
    }
}