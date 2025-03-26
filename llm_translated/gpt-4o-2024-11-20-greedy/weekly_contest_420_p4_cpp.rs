use std::io::{self, BufRead};
use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn find_answer(parent: Vec<usize>, s: String) -> Vec<bool> {
        let n = parent.len();
        let mut g = vec![Vec::new(); n];
        for i in 1..n {
            let p = parent[i];
            g[p].push(i);
        }

        let mut dfs_str = vec![0u8; n];
        let mut nodes = vec![(0, 0); n];
        let mut time = 0;

        // Depth-first search to populate `dfs_str` and `nodes`
        fn dfs(
            g: &Vec<Vec<usize>>,
            s: &str,
            dfs_str: &mut Vec<u8>,
            nodes: &mut Vec<(usize, usize)>,
            time: &mut usize,
            x: usize,
        ) {
            nodes[x].0 = *time;
            for &y in &g[x] {
                dfs(g, s, dfs_str, nodes, time, y);
            }
            dfs_str[*time] = s.as_bytes()[x];
            *time += 1;
            nodes[x].1 = *time;
        }

        dfs(&g, &s, &mut dfs_str, &mut nodes, &mut time, 0);

        // Build the transformed string for Manacher's algorithm
        let mut t = String::from("^");
        for &c in &dfs_str {
            t.push('#');
            t.push(c as char);
        }
        t.push_str("#$");

        let t_bytes = t.as_bytes();
        let mut half_len = vec![0; t_bytes.len() - 2];
        half_len[1] = 1;
        let mut box_m = 0;
        let mut box_r = 0;

        // Manacher's algorithm to calculate palindrome lengths
        for i in 2..half_len.len() {
            let mut hl = 1;
            if i < box_r {
                hl = hl.min(half_len[2 * box_m - i].min(box_r - i));
            }
            while t_bytes[i - hl] == t_bytes[i + hl] {
                hl += 1;
            }
            half_len[i] = hl;
            if i + hl > box_r {
                box_m = i;
                box_r = i + hl;
            }
        }

        // Helper function to check if a substring is a palindrome
        let is_palindrome = |l: usize, r: usize| -> bool {
            half_len[l + r + 1] > r - l
        };

        // Generate the answer
        let mut ans = vec![false; n];
        for i in 0..n {
            ans[i] = is_palindrome(nodes[i].0, nodes[i].1);
        }
        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of nodes
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read the parent array
    let parent: Vec<usize> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Read the string `s`
    let s: String = lines.next().unwrap().unwrap();

    // Solve the problem
    let solution = Solution;
    let ans = solution.find_answer(parent, s);

    // Print the result
    for &b in &ans {
        if b {
            print!("true ");
        } else {
            print!("false ");
        }
    }
    println!();
}