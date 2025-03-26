use std::io;

struct Interval {
    left: usize,
    right: usize,
}

fn max_substring_length(s: &str, k: i32) -> bool {
    let n = s.len();
    if n == 0 {
        return false;
    }

    let mut pos: Vec<Vec<usize>> = vec![Vec::new(); 26];
    for (i, c) in s.bytes().enumerate() {
        let idx = (c - b'a') as usize;
        pos[idx].push(i);
    }

    let mut intervals = Vec::new();
    for i in 0..26 {
        if !pos[i].is_empty() {
            let mut l = pos[i][0];
            let mut r = *pos[i].last().unwrap();
            let mut flag = true;
            while flag {
                flag = false;
                for j in 0..26 {
                    if !pos[j].is_empty() {
                        let low_idx = pos[j].partition_point(|&x| x < l);
                        let up_idx = pos[j].partition_point(|&x| x <= r);
                        let cnt = up_idx - low_idx;
                        if cnt > 0 && cnt < pos[j].len() {
                            let new_l = pos[j][0];
                            let new_r = *pos[j].last().unwrap();
                            l = std::cmp::min(l, new_l);
                            r = std::cmp::max(r, new_r);
                            flag = true;
                        }
                    }
                }
            }
            if l > 0 || r < n - 1 {
                intervals.push(Interval { left: l, right: r });
            }
        }
    }

    intervals.sort_by_key(|intv| intv.right);

    let mut R = -1isize;
    let mut cnt = 0;
    for intv in intervals.iter() {
        if intv.left as isize > R {
            R = intv.right as isize;
            cnt += 1;
        }
    }

    cnt >= k as usize
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");

    let mut parts = input.split_whitespace();
    let s = parts.next().expect("No string provided");
    let k = parts.next().expect("No k provided").parse::<i32>().expect("Invalid k");

    let result = max_substring_length(s, k);
    println!("{}", if result { "true" } else { "false" });
}