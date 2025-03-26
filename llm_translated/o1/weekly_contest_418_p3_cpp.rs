// Translated from the original C++ code for LeetCode Weekly Contest 418 Problem 3
// This Rust program preserves the same input/output format and algorithm logic.

use std::io::{self, BufRead, Write};

// A helper struct matching the original "Solution" concept.
struct Solution;

impl Solution {
    // Constructs a grid layout based on the given graph edges
    // preserving the same logic as the C++ version.
    fn construct_grid_layout(&self, n: usize, edges: &[Vec<usize>]) -> Vec<Vec<usize>> {
        // Build adjacency list
        let mut g = vec![Vec::new(); n];
        for e in edges {
            let x = e[0];
            let y = e[1];
            g[x].push(y);
            g[y].push(x);
        }

        // Find a node x with the smallest degree
        let mut x = 0;
        for i in 0..n {
            if g[i].len() < g[x].len() {
                x = i;
            }
        }

        // Prepare to build the first row
        let mut row = vec![x];
        let mut vis = vec![false; n];
        vis[x] = true;

        // Store the degree of the starting node
        let deg_st = g[x].len();

        // Emulate the C++ do-while structure:
        loop {
            // Find the next neighbor with the smallest degree among unvisited
            let mut nxt: isize = -1;
            for &y in &g[x] {
                if !vis[y] && (nxt < 0 || g[y].len() < g[nxt as usize].len()) {
                    nxt = y as isize;
                }
            }
            // Assign x to the found neighbor
            x = nxt as usize;
            row.push(x);
            vis[x] = true;
            // Continue while the new x still has degree > deg_st
            if g[x].len() <= deg_st {
                break;
            }
        }

        // The number of rows is n / row size
        let rows_count = n / row.len();
        let mut ans = vec![Vec::new(); rows_count];
        ans[0] = row;

        // Build each subsequent row based on unvisited neighbors of the previous row
        for i in 1..rows_count {
            for &prev_x in &ans[i - 1] {
                for &y in &g[prev_x] {
                    if !vis[y] {
                        vis[y] = true;
                        ans[i].push(y);
                        break;
                    }
                }
            }
        }

        ans
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read n and edgesSize from the first line
    let first_line = lines.next().unwrap()?;
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let edges_size: usize = parts.next().unwrap().parse().unwrap();

    // Read the edges
    let mut edges = Vec::with_capacity(edges_size);
    for _ in 0..edges_size {
        let line = lines.next().unwrap()?;
        let mut nums = line.split_whitespace();
        let x = nums.next().unwrap().parse::<usize>().unwrap();
        let y = nums.next().unwrap().parse::<usize>().unwrap();
        edges.push(vec![x, y]);
    }

    // Solve
    let solution = Solution;
    let res = solution.construct_grid_layout(n, &edges);

    // Print the result
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    for row in res {
        for x in row {
            write!(handle, "{} ", x)?;
        }
        writeln!(handle)?;
    }

    Ok(())
}