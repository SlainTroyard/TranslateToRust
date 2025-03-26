use std::io;

struct Solution;

impl Solution {
    fn maximum_subarray_xor(f: &mut Vec<i32>, queries: &Vec<Vec<i32>>) -> Vec<i32> {
        let n = f.len();
        let mut mx = vec![vec![0; n]; n];
        
        for i in (0..n).rev() {
            mx[i][i] = f[i];
            for j in (i + 1)..n {
                f[j] ^= f[j - 1];
                mx[i][j] = std::cmp::max(f[j], std::cmp::max(mx[i + 1][j], mx[i][j - 1]));
            }
        }

        let mut ans = Vec::new();
        for q in queries {
            ans.push(mx[q[0] as usize][q[1] as usize]);
        }
        ans
    }
}

fn main() {
    // Read the size of the nums array
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let nums_size: usize = input.trim().parse().unwrap();
    
    // Read the nums array
    let mut nums = Vec::with_capacity(nums_size);
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    nums = input.split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    
    // Read the number of queries
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let queries_size: usize = input.trim().parse().unwrap();
    
    // Read the queries
    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let query: Vec<i32> = input.split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        queries.push(query);
    }
    
    // Call the solution and print the result
    let result = Solution::maximum_subarray_xor(&mut nums, &queries);
    
    for (i, &val) in result.iter().enumerate() {
        print!("{}", val);
        if i < result.len() - 1 {
            print!(" ");
        }
    }
    println!();
}