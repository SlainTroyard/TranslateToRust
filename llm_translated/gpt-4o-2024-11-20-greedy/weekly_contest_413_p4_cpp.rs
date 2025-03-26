use std::io::{self, BufRead};
use std::cmp::max;

struct Solution;

impl Solution {
    pub fn maximum_subarray_xor(f: Vec<i32>, queries: Vec<Vec<usize>>) -> Vec<i32> {
        let n = f.len();
        let mut mx = vec![vec![0; n]; n];
        let mut f = f.clone(); // Clone the input vector to avoid modifying the original

        // Precompute the maximum XOR subarray values
        for i in (0..n).rev() {
            mx[i][i] = f[i];
            for j in (i + 1)..n {
                f[j] ^= f[j - 1];
                mx[i][j] = max(f[j], max(mx[i + 1][j], mx[i][j - 1]));
            }
        }

        // Process the queries
        let mut ans = Vec::new();
        for q in queries {
            ans.push(mx[q[0]][q[1]]);
        }
        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the nums array
    let nums_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read the nums array
    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Read the size of the queries array
    let queries_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read the queries array
    let mut queries = Vec::new();
    for _ in 0..queries_size {
        let query: Vec<usize> = lines
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        queries.push(query);
    }

    // Solve the problem
    let solution = Solution;
    let result = solution.maximum_subarray_xor(nums, queries);

    // Print the result
    for (i, res) in result.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", res);
    }
    println!();
}