use std::cmp::max;
use std::io;
use std::io::Read;

struct Solution {}

impl Solution {
    pub fn maximum_subarray_xor(f: Vec<int>, queries: Vec<Vec<int>>) -> Vec<int> {
        let n = f.len();
        let mut mx = vec![vec![0; n]; n];

        let mut modified_f = f.clone(); // Create a mutable copy of f

        for i in (0..n).rev() {
            mx[i][i] = modified_f[i];
            for j in i + 1..n {
                modified_f[j] ^= modified_f[j - 1];
                mx[i][j] = max(modified_f[j], max(if i + 1 < n { mx[i + 1][j] } else { 0 }, if j > 0 { mx[i][j - 1] } else { 0 }));
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
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();

    let nums_size: usize = lines.next().unwrap().parse().unwrap();
    let mut nums: Vec<int> = Vec::with_capacity(nums_size);
    for _ in 0..nums_size {
        nums.push(lines.next().unwrap().parse().unwrap());
    }

    let queries_size: usize = lines.next().unwrap().parse().unwrap();
    let mut queries: Vec<Vec<int>> = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        let line = lines.next().unwrap();
        let mut values = line.split_whitespace();
        let q1: int = values.next().unwrap().parse().unwrap();
        let q2: int = values.next().unwrap().parse().unwrap();
        queries.push(vec![q1, q2]);
    }

    let solution = Solution {};
    let result = solution.maximum_subarray_xor(nums, queries);

    for i in 0..result.len() {
        print!("{} ", result[i]);
    }
}