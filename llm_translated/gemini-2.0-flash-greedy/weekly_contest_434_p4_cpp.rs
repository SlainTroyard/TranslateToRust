use std::collections::HashSet;
use std::io;
use std::io::Read;

struct Solution {}

impl Solution {
    pub fn supersequences(words: Vec<String>) -> Vec<Vec<int>> {
        let mut all = 0;
        let mut mask2 = 0;
        let mut g: Vec<Vec<usize>> = vec![vec![]; 26];

        for s in &words {
            let x = (s.as_bytes()[0] - b'a') as usize;
            let y = (s.as_bytes()[1] - b'a') as usize;
            all |= 1 << x | 1 << y;
            if x == y {
                mask2 |= 1 << x;
            }
            g[x].push(y);
        }

        let has_cycle = |sub: i32| -> bool {
            let mut color = [0; 26];
            let mut dfs = |x: usize, dfs: &mut dyn FnMut(usize) -> bool| -> bool {
                color[x] = 1;
                for &y in &g[x] {
                    if (sub >> y & 1) == 0 {
                        continue;
                    }
                    if color[y] == 1 || (color[y] == 0 && dfs(y)) {
                        return true;
                    }
                }
                color[x] = 2;
                return false;
            };

            for i in 0..26 {
                if (sub >> i & 1) != 0 && color[i] == 0 {
                    if dfs(i, &mut dfs) {
                        return true;
                    }
                }
            }
            return false;
        };

        let mut st: HashSet<i32> = HashSet::new();
        let mut max_size = 0;
        let mask1 = all ^ mask2;
        let mut sub = mask1;

        loop {
            let size = sub.count_ones() as usize;
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

        let mut ans: Vec<Vec<int>> = Vec::new();
        for sub in st {
            let mut cnt: Vec<int> = vec![0; 26];
            for i in 0..26 {
                cnt[i] = ((all >> i & 1) + ((all ^ sub) >> i & 1)) as int;
            }
            ans.push(cnt);
        }
        return ans;
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();
    let n: usize = lines.next().unwrap().parse()?;

    let mut words: Vec<String> = Vec::new();
    for _ in 0..n {
        words.push(lines.next().unwrap().to_string());
    }

    let sol = Solution {};
    let ans = sol.supersequences(words);

    for cnt in &ans {
        for i in 0..26 {
            print!("{} ", cnt[i]);
        }
        println!();
    }

    Ok(())
}