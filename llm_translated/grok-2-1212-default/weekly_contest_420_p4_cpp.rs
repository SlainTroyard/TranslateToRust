use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn find_answer(parent: Vec<i32>, s: String) -> Vec<bool> {
        let n = parent.len();
        let mut g: Vec<Vec<usize>> = vec![Vec::new(); n];
        for i in 1..n {
            let p = parent[i] as usize;
            g[p].push(i);
        }

        let mut dfs_str = vec![0; n];
        let mut nodes: Vec<(usize, usize)> = vec![(0, 0); n];
        let mut time = 0;
        let mut dfs = |x: usize| {
            nodes[x].0 = time;
            for &y in &g[x] {
                dfs(y);
            }
            dfs_str[time] = s.as_bytes()[x] as char;
            time += 1;
            nodes[x].1 = time;
        };
        dfs(0);

        let mut t = String::from("^");
        for c in &dfs_str {
            t.push('#');
            t.push(*c);
        }
        t.push_str("#$");

        let mut half_len: Vec<usize> = vec![0; t.len() - 2];
        half_len[1] = 1;
        let (mut box_m, mut box_r) = (0, 0);
        for i in 2..half_len.len() {
            let mut hl = 1;
            if i < box_r {
                hl = half_len[box_m * 2 - i].min(box_r - i);
            }
            while t.as_bytes()[i - hl] == t.as_bytes()[i + hl] {
                hl += 1;
                box_m = i;
                box_r = i + hl;
            }
            half_len[i] = hl;
        }

        let is_palindrome = |l: usize, r: usize| -> bool {
            half_len[l + r + 1] > r - l
        };

        let mut ans = vec![false; n];
        for i in 0..n {
            ans[i] = is_palindrome(nodes[i].0, nodes[i].1);
        }
        ans
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap()?.trim().parse().unwrap();
    let parent: Vec<i32> = lines.next().unwrap()?
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let s = lines.next().unwrap()?.trim().to_string();

    let solution = Solution;
    let ans = solution.find_answer(parent, s);

    for b in ans {
        if b {
            print!("true ");
        } else {
            print!("false ");
        }
    }
    println!();

    Ok(())
}