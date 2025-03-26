use std::io;
use std::io::Read;
use std::cmp::min;

struct Solution {}

impl Solution {
    pub fn find_answer(parent: &Vec<i32>, s: String) -> Vec<bool> {
        let n = parent.len();
        let s_bytes = s.as_bytes();
        let mut g: Vec<Vec<usize>> = vec![Vec::new(); n];
        for i in 1..n {
            let p = parent[i] as usize;
            g[p].push(i);
        }

        let mut dfs_str: Vec<u8> = vec![0; n];
        let mut nodes: Vec<(usize, usize)> = vec![(0, 0); n];
        let mut time = 0;

        fn dfs(
            x: usize,
            g: &Vec<Vec<usize>>,
            s_bytes: &[u8],
            dfs_str: &mut Vec<u8>,
            nodes: &mut Vec<(usize, usize)>,
            time: &mut usize,
        ) {
            nodes[x].0 = *time;
            for &y in &g[x] {
                dfs(y, g, s_bytes, dfs_str, nodes, time);
            }
            dfs_str[*time] = s_bytes[x];
            *time += 1;
            nodes[x].1 = *time;
        }

        dfs(0, &g, s_bytes, &mut dfs_str, &mut nodes, &mut time);

        let mut t = String::from("^");
        for &c in &dfs_str {
            t.push('#');
            t.push(c as char);
        }
        t.push_str("#$");

        let mut half_len: Vec<i32> = vec![0; t.len() - 2];
        half_len[1] = 1;
        let mut box_m = 0;
        let mut box_r = 0;
        for i in 2..half_len.len() {
            let mut hl = 1;
            if i < box_r {
                hl = min(half_len[box_m * 2 - i], box_r as i32 - i as i32) as i32;
            }
            while t.as_bytes()[(i - hl as usize) as usize] == t.as_bytes()[(i + hl as usize) as usize] {
                hl += 1;
                box_m = i;
                box_r = i + hl as usize;
            }
            half_len[i] = hl - 1;
        }

        let is_palindrome = |l: usize, r: usize| -> bool {
            half_len[l + r + 1] > (r - l) as i32
        };

        let mut ans: Vec<bool> = vec![false; n];
        for i in 0..n {
            ans[i] = is_palindrome(nodes[i].0, nodes[i].1);
        }
        ans
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut lines = input.lines();

    let n: usize = lines.next().unwrap().parse().unwrap();
    let parent: Vec<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let s: String = lines.next().unwrap().to_string();

    let solution = Solution {};
    let ans = solution.find_answer(&parent, s);

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