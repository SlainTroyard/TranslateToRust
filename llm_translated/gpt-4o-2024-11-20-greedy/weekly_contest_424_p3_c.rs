use std::io::{self, BufRead};
use std::cmp::max;

fn min_zero_array(nums: &mut Vec<i32>, queries: &Vec<Vec<i32>>) -> i32 {
    let nums_size = nums.len();
    let queries_size = queries.len();

    if nums.is_empty() {
        return 0;
    }

    if queries.is_empty() {
        return -1;
    }

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
            max[end + 1] -= increment;
        }
        sum += max[i];
    }

    k as i32
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Reading input for nums
    let nums_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut nums: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Reading input for queries
    let queries_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut queries: Vec<Vec<i32>> = Vec::new();

    for _ in 0..queries_size {
        let query: Vec<i32> = lines
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        queries.push(query);
    }

    // Calling the function
    let result = min_zero_array(&mut nums, &queries);

    // Output the result
    println!("{}", result);
}