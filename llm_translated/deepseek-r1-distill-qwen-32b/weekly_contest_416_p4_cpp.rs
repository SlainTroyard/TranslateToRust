struct Solution;

impl Solution {
    fn valid_substring_count(word1: String, word2: String) -> i64 {
        let mut diff = [0i32; 26];
        for c in word2.as_bytes() {
            let idx = (c - b'a') as usize;
            if idx < 26 {
                diff[idx] -= 1;
            }
        }

        let mut cnt = diff.iter().filter(|&&x| x < 0).count() as i32;

        let mut res = 0;
        let mut r = 0;
        let n = word1.len();

        for l in 0..n {
            while r < n && cnt > 0 {
                let c = word1.as_bytes()[r] - b'a';
                let idx = c as usize;
                if idx < 26 {
                    update_diff(&mut diff, &mut cnt, idx, 1);
                }
                r += 1;
            }
            if cnt == 0 {
                res += (n - r) as i64 + 1;
            }
            let c = word1.as_bytes()[l] - b'a';
            let idx = c as usize;
            if idx < 26 {
                update_diff(&mut diff, &mut cnt, idx, -1);
            }
        }

        res
    }
}

fn update_diff(diff: &mut [i32], cnt: &mut i32, idx: usize, add: i32) {
    let old = diff[idx];
    diff[idx] += add;
    if add == 1 {
        if old == -1 && diff[idx] == 0 {
            *cnt -= 1;
        }
    } else {
        if old == 0 && diff[idx] == -1 {
            *cnt += 1;
        }
    }
}

fn main() -> std::io::Result<()> {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read len1
    let _ = lines.next().ok_or(io::Error::new(io::ErrorKind::Other, "No len1"))??;
    // Read word1
    let word1 = lines.next().ok_or(io::Error::new(io::ErrorKind::Other, "No word1"))??;
    // Read len2
    let _ = lines.next().ok_or(io::Error::new(io::ErrorKind::Other, "No len2"))??;
    // Read word2
    let word2 = lines.next().ok_or(io::Error::new(io::ErrorKind::Other, "No word2"))??;

    let solution = Solution;
    let result = solution.valid_substring_count(word1, word2);
    println!("{}", result);
    Ok(())
}