use std::io::{self, Read};

fn cal_time(mut mountain_height: i64, worker_times: &[i32], target_time: i64) -> bool {
    for &wt in worker_times {
        let worker_time = wt as i64;
        let mut l = 0i64;
        let mut r = 1_000_000i64;
        
        // Binary search for maximum blocks the worker can process in target_time
        while l <= r {
            let mid = (l + r) / 2;
            let total_time = mid * (worker_time + mid * worker_time) / 2;
            if total_time > target_time {
                r = mid - 1;
            } else {
                l = mid + 1;
            }
        }
        mountain_height -= r;  // Subtract worker's contribution
    }
    mountain_height <= 0
}

fn min_number_of_seconds(mountain_height: i32, worker_times: &[i32]) -> i64 {
    let mountain_height = mountain_height as i64;
    let mut left = 0i64;
    let mut right = 1_000_000_000_000_000_000i64;  // 1e18 as upper bound
    
    // Binary search for minimal valid time
    while left <= right {
        let mid = (left + right) / 2;
        if cal_time(mountain_height, worker_times, mid) {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }
    right + 1  // Adjust to correct minimal time after search
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut tokens = input.split_whitespace();
    
    // Parse initial input values
    let mountain_height: i32 = tokens.next().expect("Missing mountain height")
        .parse().expect("Invalid mountain height");
    let worker_count: usize = tokens.next().expect("Missing worker count")
        .parse().expect("Invalid worker count");
    
    // Parse worker times with exact count validation
    let worker_times: Vec<i32> = tokens
        .take(worker_count)
        .map(|s| s.parse().expect("Invalid worker time"))
        .collect();
    
    if worker_times.len() != worker_count {
        panic!("Insufficient worker times provided");
    }
    
    // Compute and output result
    println!("{}", min_number_of_seconds(mountain_height, &worker_times));
}