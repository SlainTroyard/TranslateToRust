// Problem: Weekly Contest 433 Problem 3
//
// This Rust program preserves the exact logic and I/O format of the original C++ code.
// It reads an integer n, then n lines each containing 3 integers for costs. It then
// computes the minimum cost using the same algorithm as the provided C++ solution,
// and prints the result to stdout.

use std::io::{self, Read};

struct Solution;

impl Solution {
    fn min_cost(n: usize, cost: &Vec<Vec<i64>>) -> i64 {
        // Create a 3D memo array, initialized to -1 (meaning "not computed yet")
        // Dimensions: (n/2) x 4 x 4
        let mut memo = vec![vec![vec![-1; 4]; 4]; n / 2];

        // Use a helper struct to do DFS with memoization
        let mut dfs = DFS::new(&mut memo, cost, n);
        // Start recursion with i = (n/2 - 1) and "pre_j=3, pre_k=3"
        dfs.dfs((n / 2 - 1) as isize, 3, 3)
    }
}

// A struct that holds mutable references to the memo and cost arrays
// and the size n. We'll define a dfs(...) method for the recursion.
struct DFS<'a> {
    memo: &'a mut Vec<Vec<Vec<i64>>>,
    cost: &'a Vec<Vec<i64>>,
    n: usize,
}

impl<'a> DFS<'a> {
    fn new(memo: &'a mut Vec<Vec<Vec<i64>>>, cost: &'a Vec<Vec<i64>>, n: usize) -> Self {
        DFS { memo, cost, n }
    }

    // Recursive DFS that returns the minimum cost for arranging pairs
    // up to index i, given previous choices pre_j and pre_k.
    fn dfs(&mut self, i: isize, pre_j: usize, pre_k: usize) -> i64 {
        // Base case: if i < 0, we've covered everything
        if i < 0 {
            return 0;
        }

        let i_usize = i as usize;

        // If we already computed this state, return it
        if self.memo[i_usize][pre_j][pre_k] != -1 {
            return self.memo[i_usize][pre_j][pre_k];
        }

        let mut min_res = i64::MAX;
        // Try all combinations of j and k with the given constraints
        for j in 0..3 {
            // j must differ from pre_j
            if j == pre_j {
                continue;
            }
            for k in 0..3 {
                // k must differ from both pre_k and j
                if k != pre_k && k != j {
                    // Recursively compute cost of subproblems
                    let temp = self.dfs(i - 1, j, k)
                        + self.cost[i_usize][j]
                        + self.cost[self.n - 1 - i_usize][k];
                    if temp < min_res {
                        min_res = temp;
                    }
                }
            }
        }

        // Store the computed result in memo before returning
        self.memo[i_usize][pre_j][pre_k] = min_res;
        min_res
    }
}

fn main() {
    // Read all input tokens as per the original code's "cin >>" usage
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let tokens: Vec<&str> = input.split_whitespace().collect();
    let mut idx = 0;

    // First integer is n
    let n: usize = tokens[idx].parse().unwrap();
    idx += 1;

    // Read cost array of shape (n x 3)
    let mut cost = vec![vec![0i64; 3]; n];
    for i in 0..n {
        for j in 0..3 {
            cost[i][j] = tokens[idx].parse().unwrap();
            idx += 1;
        }
    }

    // Solve using the same logic as in C++
    let ans = Solution::min_cost(n, &cost);

    // Print the result
    println!("{}", ans);
}