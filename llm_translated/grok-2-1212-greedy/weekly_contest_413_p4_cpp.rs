use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn maximum_subarray_xor(f: &Vec<i32>, queries: &Vec<Vec<usize>>) -> Vec<i32> {
        let n = f.len();
        let mut mx = vec![vec![0; n]; n];
        
        for i in (0..n).rev() {
            mx[i][i] = f[i];
            for j in i+1..n {
                let mut fj = f[j];
                if j > i {
                    fj ^= f[j-1];
                }
                mx[i][j] = *vec![fj, mx[i+1][j], mx[i][j-1]].iter().max().unwrap_or(&0);
            }
        }

        queries.iter().map(|q| mx[q[0]][q[1]]).collect()
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read numsSize
    let nums_size: usize = lines.next().unwrap()?.trim().parse().unwrap();
    
    // Read nums
    let nums: Vec<i32> = lines
        .by_ref()
        .take(nums_size)
        .map(|line| line.unwrap().trim().parse().unwrap())
        .collect();

    // Read queriesSize
    let queries_size: usize = lines.next().unwrap()?.trim().parse().unwrap();

    // Read queries
    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        let query: Vec<usize> = lines
            .next()
            .unwrap()?
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        queries.push(query);
    }

    // Process queries
    let result = Solution::maximum_subarray_xor(&nums, &queries);

    // Print result
    for num in result {
        print!("{} ", num);
    }
    println!();

    Ok(())
}