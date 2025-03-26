use std::io::{self, BufRead};

fn is_zero_array(nums: &[i32], queries: &[(usize, usize)]) -> bool {
    let nums_size = nums.len();
    let mut diff = vec![0; nums_size];
    let mut count = 0;

    for &(start, end) in queries {
        diff[start] += 1;
        if (end + 1) < nums_size {
            diff[end + 1] -= 1;
        }
    }

    for (i, &num) in nums.iter().enumerate() {
        count += diff[i];
        if num > count {
            return false;
        }
    }

    true
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the nums array
    let nums_size: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut nums = Vec::with_capacity(nums_size);

    // Read the elements of the nums array
    for line in lines.by_ref().take(nums_size) {
        nums.push(line.unwrap().parse().unwrap());
    }

    // Read the size of the queries array
    let queries_size: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut queries = Vec::with_capacity(queries_size);

    // Read the queries
    for line in lines.by_ref().take(queries_size) {
        let query: Vec<usize> = line.unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        queries.push((query[0], query[1]));
    }

    // Call the function to check if the array can be zeroed
    if is_zero_array(&nums, &queries) {
        println!("true");
    } else {
        println!("false");
    }
}