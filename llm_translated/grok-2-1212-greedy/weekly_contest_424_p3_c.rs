use std::io::{self, BufRead};

fn min_zero_array(nums: &mut [i32], queries: &[(usize, usize, i32)]) -> i32 {
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

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Reading input for nums
    let nums_size: usize = lines.next().unwrap()?.parse().unwrap();
    let mut nums: Vec<i32> = lines
        .by_ref()
        .take(nums_size)
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    // Reading input for queries
    let queries_size: usize = lines.next().unwrap()?.parse().unwrap();
    let mut queries: Vec<(usize, usize, i32)> = Vec::with_capacity(queries_size);

    for _ in 0..queries_size {
        let query: Vec<i32> = lines
            .by_ref()
            .take(3)
            .map(|line| line.unwrap().parse().unwrap())
            .collect();
        queries.push((query[0] as usize, query[1] as usize, query[2]));
    }

    // Calling the function
    let result = min_zero_array(&mut nums, &queries);

    // Output the result
    println!("{}", result);

    Ok(())
}