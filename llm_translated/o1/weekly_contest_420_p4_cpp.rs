// Weekly Contest 420 Problem 4 in Rust
//
// REQUIREMENTS (as per the original C++ code):
// 1. Reads n from stdin
// 2. Reads n integers for the 'parent' array
// 3. Reads a string s
// 4. Computes and prints "true " or "false " for each node
// 5. Prints a newline at the end

use std::io::{self, Read};

struct Solution;

impl Solution {
    // Translated from: vector<bool> findAnswer(vector<int>& parent, string s)
    fn find_answer(&self, parent: &[usize], s: &[char]) -> Vec<bool> {
        let n = parent.len();

        // Build adjacency list
        let mut g = vec![Vec::new(); n];
        // Just like the C++ code, we only start from i = 1
        for i in 1..n {
            let p = parent[i];
            g[p].push(i);
        }

        // Prepare arrays for DFS
        // dfs_str will store the DFS order of characters
        // nodes will store intervals (start, end) for each node in this DFS order
        let mut dfs_str = vec!['\0'; n];
        let mut nodes = vec![(0, 0); n];
        let mut time = 0;

        // DFS function replicating the C++ lambda
        fn dfs(
            x: usize,
            g: &Vec<Vec<usize>>,
            s: &[char],
            dfs_str: &mut Vec<char>,
            nodes: &mut Vec<(usize, usize)>,
            time: &mut usize,
        ) {
            // Mark the start time for this node
            nodes[x].0 = *time;
            // Visit children
            for &y in &g[x] {
                dfs(y, g, s, dfs_str, nodes, time);
            }
            // Place the character in the DFS order
            dfs_str[*time] = s[x];
            *time += 1;
            // Mark the end time for this node
            nodes[x].1 = *time;
        }

        // Run DFS from the root (0)
        dfs(0, &g, s, &mut dfs_str, &mut nodes, &mut time);

        // Construct the string for Manacher's algorithm
        // Equivalent to:
        //   string t = "^";
        //   for (char c : dfsStr) {
        //       t += '#';
        //       t += c;
        //   }
        //   t += "#$";
        let mut t = String::from("^");
        for &c in &dfs_str {
            t.push('#');
            t.push(c);
        }
        t.push_str("#$");

        // Convert to a Vec<char> for index-based access
        let t_chars: Vec<char> = t.chars().collect();
        // Prepare the halfLen array (Manacher's array)
        let mut half_len = vec![0; t_chars.len() - 2];
        // The C++ code sets halfLen[1] = 1 initially
        half_len[1] = 1;
        // boxM and boxR track the center and right boundary of the current palindrome in Manacher's
        let mut box_m = 0;
        let mut box_r = 0;

        // Manacher's algorithm (similar to the C++ code)
        for i in 2..half_len.len() {
            let mut hl = 1;
            if i < box_r {
                // Mirror index
                let mirror_i = 2 * box_m - i;
                let diff = box_r - i;
                // hl = min( halfLen[mirror_i], diff )
                if half_len[mirror_i] < diff {
                    hl = half_len[mirror_i];
                } else {
                    hl = diff;
                }
            }
            // Expand around center i
            while t_chars[i - hl] == t_chars[i + hl] {
                hl += 1;
                box_m = i;
                box_r = i + hl;
            }
            // Store the result
            half_len[i] = hl;
        }

        // Translated from: auto isPalindrome = [&](int l, int r) -> bool { return halfLen[l+r+1] > r-l; };
        let is_palindrome = |l: usize, r: usize| {
            // l + r + 1 must be a valid index within half_len
            half_len[l + r + 1] > r - l
        };

        // Compute final answers for each node
        let mut ans = vec![false; n];
        for i in 0..n {
            ans[i] = is_palindrome(nodes[i].0, nodes[i].1);
        }
        ans
    }
}

fn main() {
    // Read all input tokens
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let mut tokens = buffer.split_whitespace();

    // First token is n
    let n: usize = tokens.next().unwrap().parse().unwrap();

    // Next n tokens for parent
    let mut parent = Vec::with_capacity(n);
    for _ in 0..n {
        // The original C++ code uses 'int', but uses values as indices,
        // so we parse as usize in Rust.
        let p: usize = tokens.next().unwrap().parse().unwrap();
        parent.push(p);
    }

    // Next token is the string s
    let s_str = tokens.next().unwrap();
    let s_chars: Vec<char> = s_str.chars().collect();

    // Solve
    let solution = Solution;
    let ans = solution.find_answer(&parent, &s_chars);

    // Print results exactly as in the C++ code
    for b in ans {
        if b {
            print!("true ");
        } else {
            print!("false ");
        }
    }
    println!();
}