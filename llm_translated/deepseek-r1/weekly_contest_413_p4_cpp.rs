struct Solution;

impl Solution {
    pub fn maximum_subarray_xor(mut f: Vec<i32>, queries: &[Vec<i32>]) -> Vec<i32> {
        let n = f.len();
        let mut mx = vec![vec![0; n]; n];
        
        // Preprocess the maximum XOR subarrays using dynamic programming
        for i in (0..n).rev() {
            mx[i][i] = f[i];
            for j in (i + 1)..n {
                f[j] ^= f[j - 1];
                mx[i][j] = f[j].max(mx[i + 1][j].max(mx[i][j - 1]));
            }
        }
        
        // Answer each query by looking up the precomputed results
        queries
            .iter()
            .map(|q| mx[q[0] as usize][q[1] as usize])
            .collect()
    }
}

fn main() {
    use std::io::{self, Read, Write};

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    // Read the input numbers
    let nums_size: usize = tokens.next().unwrap().parse().unwrap();
    let nums: Vec<i32> = (0..nums_size)
        .map(|_| tokens.next().unwrap().parse().unwrap())
        .collect();

    // Read the queries
    let queries_size: usize = tokens.next().unwrap().parse().unwrap();
    let queries: Vec<Vec<i32>> = (0..queries_size)
        .map(|_| {
            let l = tokens.next().unwrap().parse().unwrap();
            let r = tokens.next().unwrap().parse().unwrap();
            vec![l, r]
        })
        .collect();

    // Compute the results
    let result = Solution::maximum_subarray_xor(nums, &queries);

    // Output the results with spaces, matching the original format
    let mut stdout = io::stdout();
    for num in result {
        write!(stdout, "{} ", num).unwrap();
    }
    stdout.flush().unwrap();
}