use std::cmp;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn maximum_subarray_xor(mut f: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = f.len();
        // Create a 2D vector to store maximum XOR values
        let mut mx = vec![vec![0; n]; n];
        
        for i in (0..n).rev() {
            mx[i][i] = f[i];
            for j in (i + 1)..n {
                // XOR operation similar to the C++ code
                f[j] ^= f[j - 1];
                // Find the maximum among three values
                mx[i][j] = cmp::max(f[j], cmp::max(
                    if i + 1 < n { mx[i + 1][j] } else { i32::MIN },
                    if j > 0 { mx[i][j - 1] } else { i32::MIN }
                ));
            }
        }

        // Process queries
        let mut ans = Vec::new();
        for q in queries {
            ans.push(mx[q[0] as usize][q[1] as usize]);
        }
        
        ans
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read nums size
    let nums_size: usize = lines.next().unwrap()?.trim().parse().unwrap();
    
    // Read nums
    let nums_line = lines.next().unwrap()?;
    let nums: Vec<i32> = nums_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Read queries size
    let queries_size: usize = lines.next().unwrap()?.trim().parse().unwrap();
    
    // Read queries
    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        let query_line = lines.next().unwrap()?;
        let query: Vec<i32> = query_line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        queries.push(query);
    }
    
    // Solve the problem
    let solution = Solution;
    let result = solution.maximum_subarray_xor(nums, queries);
    
    // Print the result
    for (i, &val) in result.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", val);
    }
    
    Ok(())
}