use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn valid_substring_count(word1: String, word2: String) -> i64 {
        let mut count = vec![0; 26];
        for c in word2.chars() {
            count[c as usize - 'a' as usize] += 1;
        }

        let n = word1.len();
        let mut pre_count = vec![vec![0; 26]; n + 1];
        for i in 1..=n {
            pre_count[i] = pre_count[i - 1].clone();
            pre_count[i][word1.chars().nth(i - 1).unwrap() as usize - 'a' as usize] += 1;
        }

        let get = |l: usize, r: usize| -> usize {
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
        };

        let mut res = 0;
        for l in 1..=n {
            let r = get(l, n + 1);
            res += (n - r + 1) as i64;
        }
        res
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let len1: usize = lines.next().unwrap()?.parse().unwrap();
    let word1: String = lines.next().unwrap()?;
    let len2: usize = lines.next().unwrap()?.parse().unwrap();
    let word2: String = lines.next().unwrap()?;

    let solution = Solution;
    let result = solution.valid_substring_count(word1, word2);
    println!("{}", result);

    Ok(())
}