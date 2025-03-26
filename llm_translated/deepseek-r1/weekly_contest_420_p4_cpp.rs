use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn find_answer(parent: Vec<i32>, s: String) -> Vec<bool> {
        let n = parent.len();
        let mut g = vec![vec![]; n];
        for i in 1..n {
            let p = parent[i] as usize;
            g[p].push(i);
        }

        let s_chars: Vec<char> = s.chars().collect();
        let mut nodes = vec![(0, 0); n];
        let mut dfs_str = Vec::with_capacity(n);
        let mut time = 0;

        // Iterative DFS to simulate post-order traversal
        let mut stack = vec![(0usize, false)];
        while let Some((x, visited)) = stack.pop() {
            if visited {
                dfs_str.push(s_chars[x]);
                nodes[x].second = time;
                time += 1;
            } else {
                nodes[x].first = time;
                stack.push((x, true));
                for &y in g[x].iter().rev() {
                    stack.push((y, false));
                }
            }
        }

        // Build transformed string for Manacher's algorithm
        let mut t = String::from("^");
        for &c in &dfs_str {
            t.push('#');
            t.push(c);
        }
        t.push_str("#$");
        let t_chars: Vec<char> = t.chars().collect();
        let len = t_chars.len();
        let mut half_len = vec![0; len.saturating_sub(2)];  // Handle empty case
        if !half_len.is_empty() {
            half_len[1] = 1;
        }
        let mut box_m = 0;
        let mut box_r = 0;

        // Manacher's algorithm to compute palindrome radii
        for i in 2..half_len.len() {
            let mut hl = 1;
            if i < box_r {
                hl = std::cmp::min(half_len[box_m * 2 - i], box_r - i);
            }
            // Expand around center while characters match
            while hl <= i && i + hl < len && t_chars[i - hl] == t_chars[i + hl] {
                hl += 1;
            }
            // Update the rightmost palindrome boundary
            if i + hl > box_r {
                box_m = i;
                box_r = i + hl;
            }
            half_len[i] = hl;
        }

        // Check if interval [l, r) forms a palindrome
        let is_palindrome = |l: usize, r: usize| {
            let center = l + r + 1;
            center < half_len.len() && half_len[center] > r - l
        };

        (0..n)
            .map(|i| {
                let (l, r) = nodes[i];
                is_palindrome(l, r)
            })
            .collect()
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());

    // Read input following the original format
    let n: usize = lines.next().unwrap().trim().parse().unwrap();
    let parent_line = lines.next().unwrap();
    let parent: Vec<i32> = parent_line
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let s = lines.next().unwrap().trim().to_string();

    // Compute answer and print in required format
    let ans = Solution::find_answer(parent, s);
    for b in ans {
        print!("{} ", if b { "true" } else { "false" });
    }
    println!();
}