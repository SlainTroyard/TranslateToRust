use std::io::{self, BufRead};

fn is_zero_array(nums: &[i32], queries: &[(usize, usize)]) -> bool {
    let nums_size = nums.len();
    let mut diff = vec![0; nums_size];
    let mut count = 0;

    for &(left, right) in queries {
        diff[left] += 1;
        if right + 1 < nums_size {
            diff[right + 1] -= 1;
        }
    }

    for i in 0..nums_size {
        count += diff[i];
        if nums[i] > count as i32 {
            return false;
        }
    }

    true
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read input for nums array size and elements
    let nums_size: usize = lines.next().unwrap()?.parse().unwrap();
    let nums: Vec<i32> = lines
        .next()
        .unwrap()?
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Read input for queries size and elements
    let queries_size: usize = lines.next().unwrap()?.parse().unwrap();
    let mut queries = Vec::with_capacity(queries_size);

    for _ in 0..queries_size {
        let query: Vec<usize> = lines
            .next()
            .unwrap()?
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        queries.push((query[0], query[1]));
    }

    // Call the function to check if the array can be zeroed
    if is_zero_array(&nums, &queries) {
        println!("true");
    } else {
        println!("false");
    }

    Ok(())
}