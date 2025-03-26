use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut stdin_lock = stdin.lock();
    let mut stdout_lock = stdout.lock();

    // Read the size of nums and then the elements of nums
    let nums_size: usize = {
        let mut input = String::new();
        stdin_lock.read_line(&mut input).unwrap();
        input.trim().parse().unwrap()
    };
    let mut nums: Vec<i32> = vec![0; nums_size];
    for num in &mut nums {
        *num = {
            let mut input = String::new();
            stdin_lock.read_line(&mut input).unwrap();
            input.trim().parse().unwrap()
        };
    }

    // Read the size of queries and then the elements of queries
    let queries_size: usize = {
        let mut input = String::new();
        stdin_lock.read_line(&mut input).unwrap();
        input.trim().parse().unwrap()
    };
    let mut queries: Vec<[usize; 2]> = vec![[0, 0]; queries_size];
    for query in &mut queries {
        let mut input = String::new();
        stdin_lock.read_line(&mut input).unwrap();
        let values: Vec<usize> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        *query = [values[0], values[1]];
    }

    // Solve the problem
    let solution = Solution {};
    let result = solution.maximum_subarray_xor(nums, queries);

    // Print the result
    for (i, &value) in result.iter().enumerate() {
        write!(stdout_lock, "{}{}", value, if i < result.len() - 1 { " " } else { "" }).unwrap();
    }
    writeln!(stdout_lock).unwrap();
}

struct Solution;

impl Solution {
    pub fn maximum_subarray_xor(f: Vec<i32>, queries: Vec<[usize; 2]>) -> Vec<i32> {
        let n = f.len();
        let mut mx = vec![vec![0; n]; n];
        let mut f = f.clone();

        for i in (0..n).rev() {
            mx[i][i] = f[i];
            for j in i + 1..n {
                f[j] ^= f[j - 1];
                mx[i][j] = *[f[j], mx[i + 1][j], mx[i][j - 1]].iter().max().unwrap();
            }
        }

        let mut ans = Vec::new();
        for q in &queries {
            ans.push(mx[q[0]][q[1]]);
        }
        ans
    }
}