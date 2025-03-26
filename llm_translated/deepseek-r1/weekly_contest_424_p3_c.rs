use std::io::{self, Read};

fn min_zero_array(nums: &[i32], queries: &[[i32; 3]]) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    if queries.is_empty() {
        return -1;
    }

    let nums_size = nums.len();
    let mut max = vec![0; nums_size + 1];
    let mut sum = 0;
    let mut k = 0;

    for i in 0..nums_size {
        // Keep applying queries until sum + max[i] >= nums[i]
        while sum + max[i] < nums[i] {
            if k >= queries.len() {
                return -1;
            }

            let [start, end, increment] = queries[k];
            let start = start as usize;
            let end = end as usize;
            k += 1;

            // Skip if current index is beyond query's end
            if i > end as usize {
                continue;
            }

            // Apply increment to the appropriate position in max array
            if i > start {
                max[i] += increment;
            } else {
                max[start] += increment;
            }

            // Subtract increment after the query's end
            let end_plus_one = end as usize + 1;
            if end_plus_one < max.len() {
                max[end_plus_one] -= increment;
            }
        }

        sum += max[i];
    }

    k as i32
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());

    // Read nums
    let nums_size = tokens.next().unwrap() as usize;
    let nums: Vec<i32> = (0..nums_size).map(|_| tokens.next().unwrap()).collect();

    // Read queries
    let queries_size = tokens.next().unwrap() as usize;
    let mut queries = Vec::with_capacity(queries_size);
    for _ in 0..queries_size {
        let start = tokens.next().unwrap();
        let end = tokens.next().unwrap();
        let inc = tokens.next().unwrap();
        queries.push([start, end, inc]);
    }

    // Process and output result
    let result = min_zero_array(&nums, &queries);
    println!("{}", result);
}