use std::collections::HashSet;
use std::io::{self, BufRead};

fn supersequences(words: Vec<String>) -> Vec<Vec<usize>> {
    let mut all = 0;
    let mut mask2 = 0;
    let mut g = vec![Vec::new(); 26];

    for s in &words {
        let x = (s.chars().nth(0).unwrap() as u8 - b'a') as usize;
        let y = (s.chars().nth(1).unwrap() as u8 - b'a') as usize;
        all |= 1 << x | 1 << y;
        if x == y {
            mask2 |= 1 << x;
        }
        g[x].push(y);
    }

    let has_cycle = |sub: usize| -> bool {
        let mut color = [0; 26];
        let mut dfs = |x: usize| -> bool {
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
            false
        };

        for i in 0..26 {
            if (sub >> i & 1) != 0 && color[i] == 0 && dfs(i) {
                return true;
            }
        }
        false
    };

    let mut st = HashSet::new();
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
        if sub == 0 {
            break;
        }
        sub = (sub - 1) & mask1;
    }

    let mut ans = Vec::new();
    for sub in st {
        let mut cnt = vec![0; 26];
        for i in 0..26 {
            cnt[i] = ((all >> i & 1) + ((all ^ sub) >> i & 1)) as usize;
        }
        ans.push(cnt);
    }
    ans
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let words: Vec<String> = (0..n)
        .map(|_| lines.next().unwrap().unwrap())
        .collect();

    let sol = supersequences(words);

    for cnt in sol {
        for (i, &val) in cnt.iter().enumerate() {
            print!("{}{}", val, if i < 25 { " " } else { "\n" });
        }
    }
}