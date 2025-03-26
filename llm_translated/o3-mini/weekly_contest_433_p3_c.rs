use std::io::{self, BufRead, Write};
use std::process;

// Helper function to read all tokens from stdin
fn read_tokens() -> Vec<String> {
    let stdin = io::stdin();
    let mut tokens = Vec::new();
    for line in stdin.lock().lines() {
        match line {
            Ok(l) => tokens.extend(l.split_whitespace().map(|s| s.to_string())),
            Err(_) => continue,
        }
    }
    tokens
}

// DFSContext holds the shared data for the DFS recursion.
// 'cost' is a reference to the full cost array (n x 3) and
// 'memo' is the 3D memoization vector with dimensions (n/2) x 4 x 4.
// We use Option<i64> where None indicates an uncomputed value.
struct DFSContext<'a> {
    cost: &'a Vec<Vec<i32>>,
    memo: Vec<Vec<Vec<Option<i64>>>>,
    n: usize,
}

impl<'a> DFSContext<'a> {
    // The recursive DFS function.
    // i: current index (using isize so that negative indicates the base condition)
    // pre_j and pre_k: previous choices, where value 3 means no previous choice.
    fn dfs(&mut self, i: isize, pre_j: usize, pre_k: usize) -> i64 {
        // Base case: when i < 0, no more pairs to process.
        if i < 0 {
            return 0;
        }
        let idx = i as usize;
        // Check memoization: if already computed, return it.
        if let Some(val) = self.memo[idx][pre_j][pre_k] {
            return val;
        }
        let mut min_res = i64::MAX;
        // Iterate over possible j choices (for the current cost row at index i).
        for j in 0..3 {
            if j == pre_j {
                continue;
            }
            // Iterate over possible k choices (for the symmetric row at index n - 1 - i).
            for k in 0..3 {
                if k != pre_k && k != j {
                    let temp = self.dfs(i - 1, j, k)
                        + self.cost[idx][j] as i64
                        + self.cost[self.n - 1 - idx][k] as i64;
                    if temp < min_res {
                        min_res = temp;
                    }
                }
            }
        }
        // Store the computed result in memoization table.
        self.memo[idx][pre_j][pre_k] = Some(min_res);
        min_res
    }
}

fn main() {
    // Read all tokens from standard input.
    let tokens = read_tokens();
    let mut iter = tokens.into_iter();

    // Parse the number of rows 'n'.
    let n: usize = match iter.next().and_then(|s| s.parse().ok()) {
        Some(num) => num,
        None => {
            // Print error and exit if we cannot read n.
            eprintln!("Error reading input for n");
            process::exit(1);
        }
    };

    // Allocate and read the cost array (n x 3).
    let mut cost: Vec<Vec<i32>> = Vec::with_capacity(n);
    for i in 0..n {
        let mut row = Vec::with_capacity(3);
        for j in 0..3 {
            let token = match iter.next() {
                Some(tok) => tok,
                None => {
                    eprintln!("Error reading input for cost[{}][{}]", i, j);
                    process::exit(1);
                }
            };
            let value: i32 = match token.parse() {
                Ok(val) => val,
                Err(_) => {
                    eprintln!("Error reading input for cost[{}][{}]", i, j);
                    process::exit(1);
                }
            };
            row.push(value);
        }
        cost.push(row);
    }

    // Calculate the size for memoization. It is n/2 (integer division).
    let m = n / 2;
    // Create a 3D vector for memoization with dimensions m x 4 x 4, 
    // initializing all values to None (uncomputed).
    let memo = vec![vec![vec![None; 4]; 4]; m];

    // Create the DFS context with cost, memo, and n.
    let mut context = DFSContext { cost: &cost, memo, n };

    // Start recursion: initial call with i = (m - 1) and previous choices 3 (none)
    // Here, in the original C code, i starts from n/2 - 1 with pre_j = pre_k = 3.
    let result = context.dfs(m as isize - 1, 3, 3);

    // Print the result to stdout.
    println!("{}", result);

    // Flush stdout to be sure output is printed.
    io::stdout().flush().unwrap();
}