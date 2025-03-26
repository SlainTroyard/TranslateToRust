use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn find_answer(parent: Vec<i32>, s: String) -> Vec<bool> {
        let n = parent.len();
        let mut g: Vec<Vec<usize>> = vec![Vec::new(); n];
        
        // Build adjacency list
        for i in 1..n {
            let p = parent[i] as usize;
            g[p].push(i);
        }

        // Initialize variables for DFS
        let mut dfs_str = vec!['\0'; n];
        let mut nodes: Vec<(usize, usize)> = vec![(0, 0); n];
        let mut time = 0;
        
        // Convert string to char vector for easier indexing
        let s_chars: Vec<char> = s.chars().collect();
        
        // DFS function to build the string and track node time ranges
        fn dfs(
            x: usize,
            g: &[Vec<usize>],
            s_chars: &[char],
            dfs_str: &mut [char],
            nodes: &mut [(usize, usize)],
            time: &mut usize,
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
        
        // Create the string for Manacher's algorithm
        let mut t = String::from("^");
        for &c in &dfs_str {
            t.push('#');
            t.push(c);
        }
        t.push('#');
        t.push('$');
        
        // Convert to char vector for efficient indexing
        let t_chars: Vec<char> = t.chars().collect();
        let mut half_len = vec![0; t_chars.len() - 2];
        half_len[1] = 1;
        let mut box_m = 0;
        let mut box_r = 0;
        
        // Manacher's algorithm to find palindromes
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
        
        // Closure to check if a substring is palindrome
        let is_palindrome = |l: usize, r: usize| -> bool {
            half_len[l + r + 1] > r - l
        };
        
        // Check for each node if its subtree forms a palindrome
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
    
    // Read n
    let n: usize = lines.next().unwrap()?.trim().parse().unwrap();
    
    // Read parent array
    let parent: Vec<i32> = lines
        .next()
        .unwrap()?
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    
    // Read string s
    let s = lines.next().unwrap()?.trim().to_string();
    
    // Get the answer and print results
    let ans = Solution::find_answer(parent, s);
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