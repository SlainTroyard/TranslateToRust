use std::io;

fn min_zero_array(nums: &[i32], queries: &[Vec<i32>]) -> i32 {
    if nums.is_empty() {
        return 0;
    }
    if queries.is_empty() {
        return -1;
    }

    let n = nums.len();
    let mut max = vec![0; n + 1];
    let mut sum = 0;
    let mut k = 0;

    for i in 0..n {
        while sum + max[i] < nums[i] {
            if k >= queries.len() {
                return -1;
            }
            let query = &queries[k];
            let start = query[0] as usize;
            let end = query[1] as usize;
            let increment = query[2];
            k += 1;

            if i > end {
                continue;
            }

            if i > start {
                max[i] += increment;
            } else {
                max[start] += increment;
            }

            if end + 1 <= n {
                max[end + 1] -= increment;
            }
        }
        sum += max[i];
    }

    k as i32
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let tokens: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    if tokens.is_empty() {
        println!(0);
        return;
    }

    let nums_size = tokens[0] as usize;
    let nums = &tokens[1..=nums_size];
    let queries_size = tokens[nums_size + 1] as usize;
    let queries = &tokens[nums_size + 2..];

    let queries_vec: Vec<Vec<i32>> = queries
        .chunks(3)
        .map(|chunk| chunk.to_vec())
        .collect();

    let result = min_zero_array(nums, &queries_vec);
    println!("{}", result);
}