use std::collections::VecDeque;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of nodes
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read the parent array
    let mut parent = Vec::with_capacity(n);
    for _ in 0..n {
        let p: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
        parent.push(p);
    }

    // Read the string
    let s = lines.next().unwrap().unwrap();

    // Create the solution and find the answer
    let solution = Solution::new();
    let ans = solution.find_answer(&parent, &s);

    // Print the result
    for b in ans {
        if b {
            print!("true ");
        } else {
            print!("false ");
        }
    }
    println!();
}

struct Solution;

impl Solution {
    pub fn new() -> Self {
        Solution
    }

    pub fn find_answer(&self, parent: &[i32], s: &str) -> Vec<bool> {
        let n = parent.len();
        let mut g = vec![Vec::new(); n];
        for i in 1..n {
            let p = parent[i] as usize;
            g[p].push(i);
        }

        let mut dfs_str = vec![0; n];
        let mut nodes = vec![(0, 0); n];
        let mut time = 0;

        let mut dfs = |x: usize| {
            let mut stack = VecDeque::new();
            stack.push_back((x, false));
            while let Some((x, visited)) = stack.pop_back() {
                if visited {
                    dfs_str[time] = s.as_bytes()[x];
                    nodes[x].1 = time + 1;
                    time += 1;
                } else {
                    nodes[x].0 = time;
                    stack.push_back((x, true));
                    for &y in &g[x] {
                        stack.push_back((y, false));
                    }
                }
            }
        };

        dfs(0);

        let mut t = String::from("^");
        for &c in &dfs_str {
            t.push('#');
            t.push(c as char);
        }
        t.push_str("#$");

        let mut half_len = vec![0; t.len() - 2];
        half_len[1] = 1;
        let mut box_m = 0;
        let mut box_r = 0;

        for i in 2..half_len.len() {
            let mut hl = 1;
            if i < box_r {
                hl = std::cmp::min(half_len[box_m * 2 - i], box_r - i);
            }
            while t.chars().nth(i - hl) == t.chars().nth(i + hl) {
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
            ans[i] = is_palindrome(nodes[i].0, nodes[i].1 - 1);
        }

        ans
    }
}