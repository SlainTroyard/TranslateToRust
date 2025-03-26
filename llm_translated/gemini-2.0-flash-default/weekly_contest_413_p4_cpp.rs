use std::cmp::max;
use std::io;
use std::io::prelude::*;

struct Solution {}

impl Solution {
    pub fn maximum_subarray_xor(
        &self,
        f: Vec<i32>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let n = f.len();
        let mut mx = vec![vec![0; n]; n];

        // Create a mutable copy of f to avoid modifying the original
        let mut mutable_f = f.clone();

        for i in (0..n).rev() {
            mx[i][i] = mutable_f[i];
            for j in i + 1..n {
                mutable_f[j] ^= mutable_f[j - 1];
                mx[i][j] = max(max(mutable_f[j], if i + 1 < n {mx[i + 1][j]} else {i32::MIN}), if j > 0{mx[i][j - 1]}else{i32::MIN});
            }
        }

        let mut ans = Vec::new();
        for q in &queries {
            ans.push(mx[q[0] as usize][q[1] as usize]);
        }
        ans
    }
}

fn main() {
    let mut nums_size = String::new();
    io::stdin().read_line(&mut nums_size).unwrap();
    let nums_size: usize = nums_size.trim().parse().unwrap();

    let mut nums_line = String::new();
    io::stdin().read_line(&mut nums_line).unwrap();
    let nums: Vec<i32> = nums_line
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut queries_size = String::new();
    io::stdin().read_line(&mut queries_size).unwrap();
    let queries_size: usize = queries_size.trim().parse().unwrap();

    let mut queries = Vec::new();
    for _ in 0..queries_size {
        let mut query_line = String::new();
        io::stdin().read_line(&mut query_line).unwrap();
        let query: Vec<i32> = query_line
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        queries.push(query);
    }

    let solution = Solution {};
    let result = solution.maximum_subarray_xor(nums, queries);

    for &val in &result {
        print!("{} ", val);
    }
}