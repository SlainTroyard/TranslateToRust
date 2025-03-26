use std::io::{self, BufRead};

fn min_zero_array(nums: &Vec<i32>, queries: &Vec<(usize, usize, i32)>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    if queries.is_empty() {
        return -1;
    }

    let mut max = vec![0; nums.len() + 1];
    let mut sum = 0;
    let mut k = 0;

    for i in 0..nums.len() {
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
    let mut nums_size = String::new();
    stdin_lock.read_line(&mut nums_size).unwrap();
    let nums_size: usize = nums_size.trim().parse().unwrap();

    let mut nums = Vec::with_capacity(nums_size);
    for _ in 0..nums_size {
        let mut num = String::new();
        stdin_lock.read_line(&mut num).unwrap();
        nums.push(num.trim().parse().unwrap());
    }

    // Reading input for queries
    let mut queries_size = String::new();
    stdin_lock.read_line(&mut queries_size).unwrap();
    let queries_size: usize = queries_size.trim().parse().unwrap();

    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        let mut query = String::new();
        stdin_lock.read_line(&mut query).unwrap();
        let parts: Vec<&str> = query.trim().split_whitespace().collect();
        let start: usize = parts[0].parse().unwrap();
        let end: usize = parts[1].parse().unwrap();
        let increment: i32 = parts[2].parse().unwrap();
        queries.push((start, end, increment));
    }

    // Calling the function
    let result = min_zero_array(&nums, &queries);

    // Output the result
    println!("{}", result);
}