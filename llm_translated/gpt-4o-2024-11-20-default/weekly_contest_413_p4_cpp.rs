use std::cmp::max;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn maximum_subarray_xor(f: Vec<i32>, queries: Vec<Vec<usize>>) -> Vec<i32> {
        let n = f.len();
        let mut mx = vec![vec![0; n]; n];
        let mut f = f.clone();

        // Compute the mx table
        for i in (0..n).rev() {
            mx[i][i] = f[i];
            for j in i + 1..n {
                f[j] ^= f[j - 1];
                mx[i][j] = max(max(f[j], mx[i + 1][j]), mx[i][j - 1]);
            }
        }

        // Process queries
        queries.iter().map(|q| mx[q[0]][q[1]]).collect()
    }
}

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();

    // Read `numsSize` and array `nums`
    let nums_size: usize = input.next().unwrap().unwrap().trim().parse().unwrap();
    let nums: Vec<i32> = input
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    // Read `queriesSize` and array `queries`
    let queries_size: usize = input.next().unwrap().unwrap().trim().parse().unwrap();
    let queries: Vec<Vec<usize>> = (0..queries_size)
        .map(|_| {
            input
                .next()
                .unwrap()
                .unwrap()
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    // Instantiate the solution and compute the result
    let solution = Solution;
    let result = solution.maximum_subarray_xor(nums, queries);

    // Output the result in the format specified
    let output = result
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", output);
}