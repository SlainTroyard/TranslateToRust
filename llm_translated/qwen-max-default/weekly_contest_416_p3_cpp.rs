use std::io::{self, BufRead, Write};

struct Solution;

impl Solution {
    fn valid_substring_count(word1: &str, word2: &str) -> i64 {
        let mut count = [0; 26];
        for c in word2.chars() {
            count[(c as u8 - b'a') as usize] += 1;
        }

        let n = word1.len();
        let mut pre_count = vec![vec![0; 26]; n + 1];
        for (i, c) in word1.chars().enumerate() {
            pre_count[i + 1] = pre_count[i].clone();
            pre_count[i + 1][(c as u8 - b'a') as usize] += 1;
        }

        let get = |l: usize, r: usize| -> usize {
            let mut l = l;
            let mut r = r;
            while l < r {
                let m = (l + r) >> 1;
                let mut f = true;
                for i in 0..26 {
                    if pre_count[m][i] - pre_count[l - 1][i] < count[i] {
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
        };

        let mut res = 0;
        for l in 1..=n {
            let r = get(l, n + 1);
            res += (n - r + 1) as i64;
        }
        res
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut stdin_lock = stdin.lock();
    let mut stdout_lock = stdout.lock();

    let mut input = String::new();
    stdin_lock.read_line(&mut input).unwrap();
    let len1: usize = input.trim().parse().unwrap();

    input.clear();
    stdin_lock.read_line(&mut input).unwrap();
    let word1 = input.trim().to_string();

    input.clear();
    stdin_lock.read_line(&mut input).unwrap();
    let len2: usize = input.trim().parse().unwrap();

    input.clear();
    stdin_lock.read_line(&mut input).unwrap();
    let word2 = input.trim().to_string();

    let s = Solution;
    let result = s.valid_substring_count(&word1, &word2);
    writeln!(stdout_lock, "{}", result).unwrap();
}