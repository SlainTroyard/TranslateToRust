use std::io;

struct Solution;

impl Solution {
    fn max_substring_length(s: &str, k: i32) -> bool {
        let n = s.len();
        if n == 0 {
            return k == 0;
        }
        let mut pos = vec![Vec::new(); 26];
        for (i, c) in s.chars().enumerate() {
            let idx = (c as usize) - ('a' as usize);
            pos[idx].push(i);
        }

        let mut intervals = Vec::new();

        for i in 0..26 {
            if pos[i].is_empty() {
                continue;
            }
            let mut l = pos[i][0];
            let mut r = *pos[i].last().unwrap();
            let mut flag = true;
            while flag {
                flag = false;
                for j in 0..26 {
                    if pos[j].is_empty() {
                        continue;
                    }
                    let j_pos = &pos[j];
                    let lower = j_pos.binary_search(&l).unwrap_or_else(|x| x);
                    let upper = j_pos.binary_search(&(r + 1)).unwrap_or_else(|x| x);
                    let cnt = upper - lower;
                    if cnt > 0 && cnt < j_pos.len() {
                        let new_l = l.min(j_pos[0]);
                        let new_r = r.max(*j_pos.last().unwrap());
                        l = new_l;
                        r = new_r;
                        flag = true;
                    }
                }
            }
            if l > 0 || r < n - 1 {
                intervals.push((r, l));
            }
        }

        intervals.sort_unstable();

        let mut R: isize = -1;
        let mut count = 0;
        for &(r, l) in &intervals {
            if l as isize > R {
                R = r as isize;
                count += 1;
            }
        }

        count >= k as usize
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    let s = parts[0];
    let k: i32 = parts[1].parse().expect("Invalid K");
    let solution = Solution;
    let result = solution.max_substring_length(s, k);
    println!("{}", if result { 1 } else { 0 });
}