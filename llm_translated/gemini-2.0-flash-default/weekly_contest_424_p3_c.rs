use std::io;
use std::io::Read;
use std::str::FromStr;

fn min_zero_array(nums: &[i32], queries: &[[i32]]) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    if queries.is_empty() {
        return -1;
    }

    let nums_size = nums.len();
    let queries_size = queries.len();

    let mut max = vec![0; nums_size + 1];

    let mut sum = 0;
    let mut k = 0;
    for i in 0..nums_size {
        while sum + max[i] < nums[i] {
            if k == queries_size {
                return -1;
            }

            let start = queries[k][0] as usize;
            let end = queries[k][1] as usize;
            let increment = queries[k][2];
            k += 1;

            if i > end {
                continue;
            }

            if i > start {
                max[i] += increment;
            } else {
                max[start] += increment;
            }
            if end + 1 < max.len() {
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

    let mut lines = input.lines();

    let nums_size: usize = lines.next().unwrap().parse().unwrap();
    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let queries_size: usize = lines.next().unwrap().parse().unwrap();
    let mut queries: Vec<Vec<i32>> = Vec::new();

    for _ in 0..queries_size {
        let query: Vec<i32> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        queries.push(query);
    }

    let result = min_zero_array(&nums, &queries);

    println!("{}", result);
}