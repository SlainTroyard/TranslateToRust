use std::io;
use std::cmp::{min, max};

fn max_substring_length(s: String, k: i32) -> bool {
    let n = s.len();

    let mut pos: Vec<Vec<usize>> = vec![Vec::new(); 26];
    for (i, c) in s.chars().enumerate() {
        let c_index = (c as u8 - b'a') as usize;
        pos[c_index].push(i);
    }

    let mut vec: Vec<(usize, usize)> = Vec::new();
    for i in 0..26 {
        if !pos[i].is_empty() {
            let mut l = pos[i][0];
            let mut r = *pos[i].last().unwrap(); // Get the last element

            let mut flag = true;
            while flag {
                flag = false;
                for j in 0..26 {
                    // Count the number of positions of character j within the range [l, r]
                    let cnt = pos[j].iter().filter(|&x| *x >= l && *x <= r).count();
                    if cnt > 0 && cnt < pos[j].len() {
                        l = min(l, pos[j][0]);
                        r = max(r, *pos[j].last().unwrap()); // Get the last element
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
    let mut r_max = -1;
    let mut cnt = 0;
    for p in vec {
        if p.1 as i32 > r_max {
            r_max = p.0 as i32;
            cnt += 1;
        }
    }
    return cnt >= k as usize;
}

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    let s = s.trim().to_string();

    let mut k_str = String::new();
    io::stdin().read_line(&mut k_str).expect("Failed to read line");
    let k: i32 = k_str.trim().parse().expect("Invalid input for K");

    let result = max_substring_length(s, k);
    println!("{}", if result { 1 } else { 0 });
}