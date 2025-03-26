use std::io::{self, BufRead, Write};

struct Solution;

impl Solution {
    pub fn maximum_subarray_xor(f: &mut Vec<i32>, queries: &Vec<Vec<i32>>) -> Vec<i32> {
        let n = f.len();
        let mut mx = vec![vec![0; n]; n];
        for i in (0..n).rev() {
            mx[i][i] = f[i];
            for j in i + 1..n {
                f[j] ^= f[j - 1];
                mx[i][j] = *[f[j], mx[i + 1][j], mx[i][j - 1]].iter().max().unwrap();
            }
        }

        queries.iter().map(|q| mx[q[0] as usize][q[1] as usize]).collect()
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut stdout = io::stdout();
    let mut stdout_lock = stdout.lock();

    // Read the size of the nums array
    let mut input = String::new();
    stdin_lock.read_line(&mut input).unwrap();
    let nums_size: usize = input.trim().parse().expect("Invalid input");

    // Read the nums array
    input.clear();
    stdin_lock.read_line(&mut input).unwrap();
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid input"))
        .collect();

    // Read the size of the queries array
    input.clear();
    stdin_lock.read_line(&mut input).unwrap();
    let queries_size: usize = input.trim().parse().expect("Invalid input");

    // Read the queries
    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        input.clear();
        stdin_lock.read_line(&mut input).unwrap();
        let q: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().expect("Invalid input"))
            .collect();
        queries.push(q);
    }

    // Solve the problem
    let mut f = nums.clone();
    let result = Solution::maximum_subarray_xor(&mut f, &queries);

    // Output the result
    for (i, &val) in result.iter().enumerate() {
        if i > 0 {
            write!(&mut stdout_lock, " ").unwrap();
        }
        write!(&mut stdout_lock, "{}", val).unwrap();
    }
    writeln!(&mut stdout_lock).unwrap();
}