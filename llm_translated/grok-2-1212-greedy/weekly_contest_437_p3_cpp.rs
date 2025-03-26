use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn max_substring_length(s: String, k: i32) -> bool {
        let n = s.len();
        let mut pos: Vec<Vec<usize>> = vec![Vec::new(); 26];

        for (i, c) in s.chars().enumerate() {
            let c = (c as u8 - b'a') as usize;
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
                        let cnt = pos[j].partition_point(|&x| x <= r) - pos[j].partition_point(|&x| x < l);
                        if cnt > 0 && cnt < pos[j].len() {
                            l = l.min(pos[j][0]);
                            r = r.max(*pos[j].last().unwrap());
                            flag = true;
                        }
                    }
                }
                if l > 0 || r < n - 1 {
                    vec.push((r, l));
                }
            }
        }

        vec.sort_unstable();
        let mut r = -1;
        let mut cnt = 0;
        for &(r_new, l) in &vec {
            if l as i32 > r {
                r = r_new as i32;
                cnt += 1;
            }
        }
        cnt >= k
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let s = lines.next().unwrap()?;
    let k: i32 = lines.next().unwrap()?.parse().unwrap();

    let sol = Solution;
    println!("{}", sol.max_substring_length(s, k));

    Ok(())
}