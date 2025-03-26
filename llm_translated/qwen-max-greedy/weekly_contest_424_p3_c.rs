use std::io::{self, BufRead, Write};

fn min_zero_array(nums: &Vec<i32>, queries: &Vec<(usize, usize, i32)>) -> i32 {
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
            if k == queries.len() {
                return -1;
            }

            let (start, end, increment) = queries[k];
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
    // Reading input for nums
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut buffer = String::new();
    stdin_lock.read_line(&mut buffer).unwrap();
    let nums_size: usize = buffer.trim().parse().unwrap();

    buffer.clear();
    let mut nums = Vec::with_capacity(nums_size);
    for _ in 0..nums_size {
        buffer.clear();
        stdin_lock.read_line(&mut buffer).unwrap();
        nums.push(buffer.trim().parse().unwrap());
    }

    // Reading input for queries
    buffer.clear();
    stdin_lock.read_line(&mut buffer).unwrap();
    let queries_size: usize = buffer.trim().parse().unwrap();

    buffer.clear();
    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        buffer.clear();
        stdin_lock.read_line(&mut buffer).unwrap();
        let mut iter = buffer.trim().split_whitespace();
        let start: usize = iter.next().unwrap().parse().unwrap();
        let end: usize = iter.next().unwrap().parse().unwrap();
        let increment: i32 = iter.next().unwrap().parse().unwrap();
        queries.push((start, end, increment));
    }

    // Calling the function
    let result = min_zero_array(&nums, &queries);

    // Output the result
    println!("{}", result);
}