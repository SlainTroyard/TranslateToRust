use std::io::{self, BufRead};
use std::cmp::min;

struct Solution;

impl Solution {
    pub fn find_answer(parent: Vec<i32>, s: String) -> Vec<bool> {
        let n = parent.len();
        let mut g = vec![Vec::new(); n];
        
        // Build the graph representation of the tree
        for (i, &p) in parent.iter().enumerate().skip(1) {
            g[p as usize].push(i);
        }

        let mut dfs_str = vec![0u8; n];
        let mut nodes = vec![(0, 0); n];
        let mut time = 0;

        // DFS function to compute preorder traversal and nodes range (start, end)
        fn dfs(
            g: &Vec<Vec<usize>>,
            s: &Vec<u8>,
            dfs_str: &mut Vec<u8>,
            nodes: &mut Vec<(i32, i32)>,
            time: &mut i32,
            x: usize,
        ) {
            nodes[x].0 = *time;
            for &y in &g[x] {
                dfs(g, s, dfs_str, nodes, time, y);
            }
            dfs_str[*time as usize] = s[x];
            *time += 1;
            nodes[x].1 = *time;
        }
        dfs(&g, &s.as_bytes().to_vec(), &mut dfs_str, &mut nodes, &mut time, 0);

        // Construct t string for Manacher's algorithm
        let mut t = "^".to_string();
        for &c in &dfs_str {
            t.push('#');
            t.push(c as char);
        }
        t.push_str("#$");

        // Manacher's algorithm to compute the halfLen array
        let t_bytes = t.as_bytes();
        let mut half_len = vec![0; t_bytes.len() - 2];
        half_len[1] = 1;
        let mut box_m = 0;
        let mut box_r = 0;

        for i in 2..half_len.len() {
            let mut hl = 1;
            if i < box_r {
                hl = min(half_len[2 * box_m - i], box_r - i);
            }
            while t_bytes[i - hl] == t_bytes[i + hl] {
                hl += 1;
            }
            if i + hl > box_r {
                box_m = i;
                box_r = i + hl;
            }
            half_len[i] = hl;
        }

        // Function to check if a substring is a palindrome
        let is_palindrome = |l: i32, r: i32| -> bool {
            half_len[(l + r + 1) as usize] > r - l
        };

        // Build the answer vector
        let mut ans = vec![false; n];
        for (i, &(start, end)) in nodes.iter().enumerate() {
            ans[i] = is_palindrome(start, end);
        }
        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Parse the number of nodes
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Parse the parent array
    let parent_line = lines.next().unwrap().unwrap();
    let parent: Vec<i32> = parent_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Parse the string s
    let s = lines.next().unwrap().unwrap();

    // Compute the answer using the Solution struct
    let solution = Solution;
    let ans = solution.find_answer(parent, s);

    // Print the results in the required format
    for b in ans {
        if b {
            print!("true ");
        } else {
            print!("false ");
        }
    }
    println!();
}