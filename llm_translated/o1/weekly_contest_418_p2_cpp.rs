// Weekly Contest 418 Problem 2 in Rust
// -----------------------------------
// This Rust program reads input for n, k, and the number of invocations,
// then reads the invocations, applies the same logic as the original C++ code,
// and prints the resulting methods in the same format.

// We use idiomatic Rust with proper error handling but maintain
// identical input and output flows to match the original C++ program.

use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn remaining_methods(
        &self,
        n: usize,
        k: usize,
        invocations: &[(usize, usize)],
    ) -> Vec<i32> {
        // Build adjacency list
        let mut g = vec![Vec::new(); n];
        for &(from, to) in invocations {
            g[from].push(to);
        }

        // "is_suspicious" tracks whether a method is suspicious
        let mut is_suspicious = vec![false; n];

        // DFS to mark all methods reachable from k (these become suspicious)
        fn dfs(x: usize, graph: &[Vec<usize>], suspicious: &mut [bool]) {
            suspicious[x] = true;
            for &next in &graph[x] {
                if !suspicious[next] {
                    dfs(next, graph, suspicious);
                }
            }
        }
        dfs(k, &g, &mut is_suspicious);

        // Check if there exists a NON-suspicious -> suspicious edge
        // If yes, we cannot remove the suspicious methods
        for &(from, to) in invocations {
            if !is_suspicious[from] && is_suspicious[to] {
                // Cannot remove suspicious methods, return all 0..n
                let mut ans = Vec::with_capacity(n);
                for i in 0..n {
                    ans.push(i as i32);
                }
                return ans;
            }
        }

        // Otherwise, remove all suspicious methods
        let mut ans = Vec::new();
        for i in 0..n {
            if !is_suspicious[i] {
                ans.push(i as i32);
            }
        }
        ans
    }
}

fn main() -> io::Result<()> {
    // Read n, k, and the number of invocations
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line)?;
    let mut parts = input_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let k: usize = parts.next().unwrap().parse().unwrap();
    let invocations_size: usize = parts.next().unwrap().parse().unwrap();

    // Read the invocations
    let stdin = io::stdin();
    let mut invocations = Vec::with_capacity(invocations_size);
    for _ in 0..invocations_size {
        let mut line = String::new();
        stdin.read_line(&mut line)?;
        let mut nums = line.split_whitespace();
        let a: usize = nums.next().unwrap().parse().unwrap();
        let b: usize = nums.next().unwrap().parse().unwrap();
        invocations.push((a, b));
    }

    // Solve using the same algorithm
    let solution = Solution;
    let ans = solution.remaining_methods(n, k, &invocations);

    // Print the results on one line, space-separated
    for x in ans {
        print!("{} ", x);
    }
    println!();

    Ok(())
}