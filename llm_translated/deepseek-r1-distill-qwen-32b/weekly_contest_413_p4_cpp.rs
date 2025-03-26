use std::io;

struct Solution;

impl Solution {
    fn maximum_subarray_xor(f: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = f.len();
        let mut mx = vec![vec![0; n]; n];
        let mut f_copy = f.clone();

        for i in (0..n).rev() {
            mx[i][i] = f_copy[i];
            for j in (i + 1)..n {
                f_copy[j] ^= f_copy[j - 1];
                mx[i][j] = f_copy[j].max(mx[i + 1][j]).max(mx[i][j - 1]);
            }
        }

        queries
            .into_iter()
            .map(|q| {
                let l = q[0] as usize;
                let r = q[1] as usize;
                mx[l][r]
            })
            .collect()
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let nums_size: usize = input.trim().parse().expect("Invalid input");

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read nums");
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid nums"))
        .collect();

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read queries size");
    let queries_size: usize = input.trim().parse().expect("Invalid queries size");

    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read query");
        let parts: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().expect("Invalid query"))
            .collect();
        queries.push(parts);
    }

    let solution = Solution;
    let result = solution.maximum_subarray_xor(nums, queries);

    for num in result {
        print!("{} ", num);
    }
    println!();
}