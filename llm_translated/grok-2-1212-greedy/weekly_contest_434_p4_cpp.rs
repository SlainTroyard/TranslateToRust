use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap()?.parse().unwrap();
    let mut words = Vec::with_capacity(n);
    for _ in 0..n {
        words.push(lines.next().unwrap()?);
    }

    let solution = Solution::new();
    let ans = solution.supersequences(&words);

    for cnt in ans {
        for &count in &cnt {
            print!("{} ", count);
        }
        println!();
    }

    Ok(())
}

struct Solution;

impl Solution {
    fn supersequences(&self, words: &[String]) -> Vec<Vec<i32>> {
        let mut all = 0;
        let mut mask2 = 0;
        let mut g = vec![Vec::new(); 26];

        for s in words {
            let x = s.as_bytes()[0] - b'a';
            let y = s.as_bytes()[1] - b'a';
            all |= 1 << x | 1 << y;
            if x == y {
                mask2 |= 1 << x;
            }
            g[x as usize].push(y);
        }

        let has_cycle = |sub: i32| -> bool {
            let mut color = vec![0; 26];
            let mut dfs = |x: i32| -> bool {
                color[x as usize] = 1;
                for &y in &g[x as usize] {
                    if (sub >> y & 1) == 0 {
                        continue;
                    }
                    if color[y as usize] == 1 || (color[y as usize] == 0 && dfs(y)) {
                        return true;
                    }
                }
                color[x as usize] = 2;
                false
            };

            for i in 0..26 {
                if (sub >> i & 1) != 0 && color[i] == 0 && dfs(i as i32) {
                    return true;
                }
            }
            false
        };

        let mut st = std::collections::HashSet::new();
        let mut max_size = 0;
        let mask1 = all ^ mask2;
        let mut sub = mask1;
        loop {
            let size = sub.count_ones() as i32;
            if size >= max_size && !has_cycle(sub) {
                if size > max_size {
                    max_size = size;
                    st.clear();
                }
                st.insert(sub);
            }
            sub = (sub - 1) & mask1;
            if sub == mask1 {
                break;
            }
        }

        let mut ans = Vec::new();
        for &sub in &st {
            let mut cnt = vec![0; 26];
            for i in 0..26 {
                cnt[i] = ((all >> i & 1) + ((all ^ sub) >> i & 1)) as i32;
            }
            ans.push(cnt);
        }
        ans
    }
}