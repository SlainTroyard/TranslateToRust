use std::io;
use std::vec::Vec;
use std::cmp;

struct Solution {}

impl Solution {
    pub fn maximum_subarray_xor(&self, f: &mut Vec<i32>, queries: &Vec<Vec<i32>>) -> Vec<i32> {
        let n = f.len();
        let mut mx: Vec<Vec<i32>> = vec![vec![0; n]; n];

        for i in (0..n).rev() {
            mx[i][i] = f[i];
            for j in i + 1..n {
                f[j] ^= f[j - 1]; // IMPORTANT: In-place modification of f as in the original C++ code
                mx[i][j] = cmp::max(f[j], cmp::max(if i + 1 < n { mx[i + 1][j] } else { 0 }, if j > 0 { mx[i][j - 1] } else { 0 }));
                 // Handle potential out-of-bounds access for mx[i+1][j] and mx[i][j-1]
                 // although in this specific loop structure, it's actually never out of bound when accessed within the loop range.
                 // The conditions are added for clarity and to match the intended logic (even if the logic is flawed) if boundaries were a real concern.
            }
        }

        let mut ans: Vec<i32> = Vec::new();
        for q in queries {
            ans.push(mx[q[0] as usize][q[1] as usize]);
        }
        ans
    }
}

fn main() {
    let mut nums_size_str = String::new();
    io::stdin().read_line(&mut nums_size_str).unwrap();
    let nums_size: usize = nums_size_str.trim().parse().unwrap();

    let mut nums_str = String::new();
    io::stdin().read_line(&mut nums_str).unwrap();
    let nums: Vec<i32> = nums_str
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut queries_size_str = String::new();
    io::stdin().read_line(&mut queries_size_str).unwrap();
    let queries_size: usize = queries_size_str.trim().parse().unwrap();

    let mut queries: Vec<Vec<i32>> = Vec::new();
    for _ in 0..queries_size {
        let mut query_str = String::new();
        io::stdin().read_line(&mut query_str).unwrap();
        let query: Vec<i32> = query_str
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        queries.push(query);
    }

    let solution = Solution {};
    // Create a mutable copy of nums to match the function signature of maximum_subarray_xor which takes &mut Vec<i32>
    let mut nums_clone = nums.clone();
    let result = solution.maximum_subarray_xor(&mut nums_clone, &queries);

    for i in 0..result.len() {
        print!("{} ", result[i]);
    }
    println!();
}