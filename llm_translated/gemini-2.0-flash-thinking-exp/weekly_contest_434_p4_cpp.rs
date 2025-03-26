use std::io;
use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn supersequences(words: Vec<String>) -> Vec<Vec<i32>> {
        let mut all: u32 = 0;
        let mut mask2: u32 = 0;
        let mut g: [Vec<usize>; 26] = Default::default();

        for s in &words {
            let x = (s.chars().nth(0).unwrap() as u8 - b'a') as usize;
            let y = (s.chars().nth(1).unwrap() as u8 - b'a') as usize;
            all |= 1 << x | 1 << y;
            if x == y {
                mask2 |= 1 << x;
            }
            g[x].push(y);
        }

        let has_cycle = |sub: u32, graph: &[Vec<usize>; 26]| -> bool {
            let mut color = [0; 26];
            let mut dfs = |x: usize, current_color: &mut [i32]| -> bool {
                current_color[x] = 1;
                for &y in &graph[x] {
                    if (sub >> y & 1) == 0 {
                        continue;
                    }
                    if current_color[y] == 1 || (current_color[y] == 0 && dfs(y, current_color)) {
                        return true;
                    }
                }
                current_color[x] = 2;
                false
            };

            for i in 0..26 {
                if (sub >> i & 1) != 0 && color[i] == 0 && dfs(i, &mut color) {
                    return true;
                }
            }
            false
        };

        let mut st: HashSet<u32> = HashSet::new();
        let mut max_size = 0;
        let mask1 = all ^ mask2;
        let mut sub = mask1;

        loop {
            let size = sub.count_ones();
            if size >= max_size && !has_cycle(sub, &g) {
                if size > max_size {
                    max_size = size;
                    st.clear();
                }
                st.insert(sub);
            }
            if sub == 0 {
                break;
            }
            sub = (sub - 1) & mask1;
        }

        let mut ans: Vec<Vec<i32>> = Vec::new();
        for sub_val in st {
            let mut cnt: Vec<i32> = vec![0; 26];
            for i in 0..26 {
                cnt[i] = ((all >> i & 1) + ((all ^ sub_val) >> i & 1)) as i32;
            }
            ans.push(cnt);
        }
        ans
    }
}

fn main() {
    let mut n_str = String::new();
    io::stdin().read_line(&mut n_str).unwrap();
    let n: usize = n_str.trim().parse().unwrap();

    let mut words: Vec<String> = Vec::with_capacity(n);
    for _ in 0..n {
        let mut word = String::new();
        io::stdin().read_line(&mut word).unwrap();
        words.push(word.trim().to_string());
    }

    let sol = Solution {};
    let ans = sol.supersequences(words);

    for cnt in &ans {
        for i in 0..26 {
            print!("{} ", cnt[i]);
        }
        println!();
    }
}