use std::io::{self, BufRead};
use std::collections::{HashSet, VecDeque};

fn supersequences(words: &[String]) -> Vec<Vec<i32>> {
    let mut all = 0;
    let mut mask2 = 0;
    let mut g = vec![Vec::new(); 26];

    for s in words {
        let x = s.chars().nth(0).unwrap() as usize - 'a' as usize;
        let y = s.chars().nth(1).unwrap() as usize - 'a' as usize;
        all |= 1 << x | 1 << y;
        if x == y {
            mask2 |= 1 << x;
        }
        g[x].push(y);
    }

    fn has_cycle(sub: i32, g: &[Vec<usize>]) -> bool {
        let mut color = vec![0; 26];
        fn dfs(x: usize, sub: i32, color: &mut [i32], g: &[Vec<usize>]) -> bool {
            color[x] = 1;
            for &y in &g[x] {
                if (sub >> y & 1) == 0 {
                    continue;
                }
                if color[y] == 1 || (color[y] == 0 && dfs(y, sub, color, g)) {
                    return true;
                }
            }
            color[x] = 2;
            false
        }
        for i in 0..26 {
            if (sub >> i & 1) != 0 && color[i] == 0 && dfs(i, sub, &mut color, g) {
                return true;
            }
        }
        false
    }

    let mut st: HashSet<i32> = HashSet::new();
    let mut max_size = 0;
    let mask1 = all ^ mask2;
    let mut sub = mask1;
    loop {
        let size = sub.count_ones();
        if size >= max_size as u32 && !has_cycle(sub, &g) {
            if size > max_size as u32 {
                max_size = size as i32;
                st.clear();
            }
            st.insert(sub);
        }
        if sub == mask1 {
            break;
        }
        sub = (sub - 1) & mask1;
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

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap()?.trim().parse().unwrap();
    let mut words = Vec::with_capacity(n);

    for _ in 0..n {
        words.push(lines.next().unwrap()?.trim().to_string());
    }

    let ans = supersequences(&words);

    for cnt in ans {
        for i in 0..26 {
            print!("{} ", cnt[i]);
        }
        println!();
    }

    Ok(())
}