use std::io;

/// Translated from the C code's minZeroArray function.
///
/// # Arguments
/// * `nums` - The input array of integers.
/// * `queries` - A list of queries, each with start, end, and increment values.
///
/// # Returns
/// The number of queries processed, or -1 if insufficient queries.
fn min_zero_array(nums: &[i32], queries: &[[i32; 3]]) -> i32 {
    let nums_size = nums.len();
    if nums_size == 0 {
        return 0;
    }
    let queries_size = queries.len();
    if queries_size == 0 {
        return -1;
    }

    let mut max = vec![0; nums_size + 1]; // +1 to handle end+1 up to nums_size
    let mut sum = 0;
    let mut k = 0;

    for i in 0..nums_size {
        while sum + max[i] < nums[i] {
            if k == queries_size {
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
            max[end + 1] -= increment;
        }
        sum += max[i];
    }

    k as i32
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut tokens = input
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let nums_size = tokens.next().unwrap();
    let nums: Vec<i32> = (0..nums_size)
        .map(|_| tokens.next().unwrap())
        .collect();

    let queries_size = tokens.next().unwrap();
    let mut queries = Vec::with_capacity(queries_size as usize);

    for _ in 0..queries_size {
        let start = tokens.next().unwrap();
        let end = tokens.next().unwrap();
        let increment = tokens.next().unwrap();
        queries.push([start, end, increment]);
    }

    let result = min_zero_array(&nums, &queries);
    println!("{}", result);
}