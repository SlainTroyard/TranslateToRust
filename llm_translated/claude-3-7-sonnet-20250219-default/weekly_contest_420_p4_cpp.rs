use std::io;
use std::io::prelude::*;

struct Solution;

impl Solution {
    pub fn find_answer(parent: Vec<i32>, s: String) -> Vec<bool> {
        let n = parent.len();
        let mut g: Vec<Vec<usize>> = vec![Vec::new(); n];
        
        // Build the tree
        for i in 1..n {
            let p = parent[i] as usize;
            g[p].push(i);
        }

        // Convert string to characters for easier indexing
        let s_chars: Vec<char> = s.chars().collect();
        let mut dfs_str = vec![' '; n];
        let mut nodes: Vec<(usize, usize)> = vec![(0, 0); n];
        let mut time = 0;

        // Define and execute DFS to build sequence
        fn dfs(
            x: usize, 
            g: &Vec<Vec<usize>>, 
            s_chars: &Vec<char>, 
            dfs_str: &mut Vec<char>,
            nodes: &mut Vec<(usize, usize)>, 
            time: &mut usize
        ) {
            nodes[x].0 = *time;
            for &y in &g[x] {
                dfs(y, g, s_chars, dfs_str, nodes, time);
            }
            dfs_str[*time] = s_chars[x];
            *time += 1;
            nodes[x].1 = *time;
        }
        
        dfs(0, &g, &s_chars, &mut dfs_str, &mut nodes, &mut time);

        // Convert dfs_str to a string
        let dfs_str: String = dfs_str.into_iter().collect();

        // Construct string for Manacher's algorithm
        let mut t = String::from("^");
        for c in dfs_str.chars() {
            t.push('#');
            t.push(c);
        }
        t.push('#');
        t.push('$');

        // Manacher's algorithm to find palindromes
        let t_chars: Vec<char> = t.chars().collect();
        let mut half_len = vec![0; t.len() - 2];
        half_len[1] = 1;
        let mut box_m = 0;
        let mut box_r = 0;
        
        for i in 2..half_len.len() {
            let mut hl = 1;
            if i < box_r {
                hl = std::cmp::min(half_len[box_m * 2 - i], box_r - i);
            }
            while t_chars[i - hl] == t_chars[i + hl] {
                hl += 1;
                box_m = i;
                box_r = i + hl;
            }
            half_len[i] = hl;
        }

        // Check if a substring is a palindrome using Manacher's results
        let is_palindrome = |l: usize, r: usize| -> bool {
            half_len[l + r + 1] > r - l
        };

        // Determine answer for each node
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
    
    // Read the size n
    let n: usize = lines.next().unwrap()?.trim().parse().unwrap();
    
    // Read the parent array
    let parent: Vec<i32> = lines.next().unwrap()?.trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Read the string s
    let s = lines.next().unwrap()?.trim().to_string();
    
    // Solve and output the result
    let ans = Solution::find_answer(parent, s);
    
    // Output the result in the same format as the original code
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