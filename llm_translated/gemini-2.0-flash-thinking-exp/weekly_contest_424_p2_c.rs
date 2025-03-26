use std::io;
use std::io::BufRead;

fn is_zero_array(nums: &[i32], queries: &[[i32]]) -> bool {
    let nums_size = nums.len();
    let mut diff = vec![0; nums_size];

    for query in queries {
        let l = query[0] as usize;
        diff[l] += 1;
        let r = query[1] as usize;
        if r + 1 < nums_size {
            diff[r + 1] -= 1;
        }
    }

    let mut count = 0;
    for i in 0..nums_size {
        count += diff[i];
        if nums[i] > count {
            return false;
        }
    }

    true
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read input for nums array size and elements
    let nums_size: usize = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .expect("Invalid input for numsSize");

    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input for nums element"))
        .collect();

    // Read input for queries size and elements
    let queries_size: usize = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .expect("Invalid input for queriesSize");

    let mut queries: Vec<Vec<i32>> = Vec::new();
    for _ in 0..queries_size {
        let query: Vec<i32> = lines
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid input for query element"))
            .collect();
        queries.push(query);
    }

    // Call the function to check if the array can be zeroed
    if is_zero_array(&nums, &queries) {
        println!("true");
    } else {
        println!("false");
    }
}