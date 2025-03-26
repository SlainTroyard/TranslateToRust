use std::io::{self, BufRead, Write};

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Parse the first line for n, k, and invocationsSize
    let first_line = lines.next().unwrap().unwrap();
    let mut iter = first_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: usize = iter.next().unwrap().parse().unwrap();
    let invocations_size: usize = iter.next().unwrap().parse().unwrap();

    // Parse the invocations
    let mut invocations = vec![vec![0; 2]; invocations_size];
    for i in 0..invocations_size {
        let line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        invocations[i][0] = iter.next().unwrap().parse().unwrap();
        invocations[i][1] = iter.next().unwrap().parse().unwrap();
    }

    // Create a Solution instance and call the remaining_methods function
    let solution = Solution;
    let ans = solution.remaining_methods(n, k, &invocations);

    // Output the result to stdout
    for x in ans {
        print!("{} ", x);
    }
    println!();
}

struct Solution;

impl Solution {
    pub fn remaining_methods(&self, n: usize, k: usize, invocations: &Vec<Vec<usize>>) -> Vec<usize> {
        // Build the graph
        let mut g = vec![vec![]; n];
        for e in invocations {
            g[e[0]].push(e[1]);
        }

        // Mark all suspicious methods
        let mut is_suspicious = vec![false; n];
        let dfs = |dfs: &dyn Fn(&dyn Fn(&dyn Fn(&mut Vec<bool>, usize) -> (), usize), usize), x: usize| {
            is_suspicious[x] = true;
            for &y in &g[x] {
                if !is_suspicious[y] { // Avoid infinite recursion
                    dfs(dfs, y);
                }
            }
        };
        dfs(&dfs, k);

        // Check if there is an edge from a non-suspicious method to a suspicious method
        for e in invocations {
            if !is_suspicious[e[0]] && is_suspicious[e[1]] {
                // Cannot remove suspicious methods
                let mut ans = (0..n).collect::<Vec<usize>>();
                return ans;
            }
        }

        // Remove all suspicious methods
        let ans = (0..n).filter(|&i| !is_suspicious[i]).collect::<Vec<usize>>();
        ans
    }
}