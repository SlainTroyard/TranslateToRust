use std::cmp;
use std::i64;
use std::io::{self, BufRead, Write};

// We'll define a struct Solution with the main method "min_cost"
// that mimics the C++ solution logic.
struct Solution;

impl Solution {
    // The function min_cost receives:
    // - n: the total number of elements in the cost vector,
    // - cost: a 2D vector of costs with n rows and 3 columns.
    // It returns the minimum cost as computed by the DFS + memoization.
    pub fn min_cost(n: i32, cost: Vec<Vec<i32>>) -> i64 {
        let half = (n / 2) as usize;
        // Create a 3D memoization table with dimensions [half][4][4],
        // using Option to mark whether a subproblem has been computed.
        // The extra index 3 in the second and third dimensions represents "no previous selection".
        let mut memo = vec![vec![vec![None; 4]; 4]; half];

        // Define a recursive DFS function that takes:
        // i: the current index (in the range [0 .. half-1]); i can be negative, which is our base case.
        // pre_j: the previous color chosen for the first house of the pair,
        // pre_k: the previous color chosen for the second house of the pair.
        //
        // The function returns the minimum cost computed for all pairs up to index i.
        fn dfs(
            i: i32,
            pre_j: usize,
            pre_k: usize,
            n: i32,
            cost: &Vec<Vec<i32>>,
            memo: &mut Vec<Vec<Vec<Option<i64>>>>,
        ) -> i64 {
            if i < 0 {
                return 0;
            }

            let idx = i as usize;
            if let Some(res) = memo[idx][pre_j][pre_k] {
                return res;
            }

            let mut min_res = i64::MAX;
            // Try each possible option for the left side color (j) and right side color (k)
            for j in 0..3 {
                if j == pre_j {
                    continue;
                }
                for k in 0..3 {
                    // Ensure that the cost selections are for different colors and respect previous selections.
                    if k == pre_k || k == j {
                        continue;
                    }
                    let temp = dfs(i - 1, j, k, n, cost, memo)
                        + cost[idx][j] as i64
                        + cost[(n - 1 - i) as usize][k] as i64;
                    min_res = cmp::min(min_res, temp);
                }
            }
            memo[idx][pre_j][pre_k] = Some(min_res);
            min_res
        }

        // Initialize recursion with i = half-1 and no previous selections (represented by index 3)
        dfs(half as i32 - 1, 3, 3, n, &cost, &mut memo)
    }
}

fn main() -> io::Result<()> {
    // Set up input reading from stdin.
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut buffer = String::new();

    // Read the first token which is the integer n.
    reader.read_line(&mut buffer)?;
    let n: i32 = buffer.trim().parse().expect("Failed to parse n");
    buffer.clear();

    // Read n lines each containing 3 integers.
    let mut cost = Vec::with_capacity(n as usize);
    for _ in 0..n {
        reader.read_line(&mut buffer)?;
        // Split the line by whitespace, parse each token into an integer.
        let row: Vec<i32> = buffer
            .split_whitespace()
            .map(|token| token.parse().expect("Failed to parse cost"))
            .collect();
        if row.len() != 3 {
            panic!("Each line must contain exactly 3 integers.");
        }
        cost.push(row);
        buffer.clear();
    }

    // Call the solution and write the result to stdout.
    let result = Solution::min_cost(n, cost);
    writeln!(io::stdout(), "{}", result)?;
    Ok(())
}