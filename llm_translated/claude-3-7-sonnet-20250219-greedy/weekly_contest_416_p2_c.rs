use std::io::{self, BufRead};

/// Calculates whether the mountain can be removed within the target time
fn cal_time(mountain_height: i64, worker_times: &[i32], target_time: i64) -> bool {
    let mut remaining_height = mountain_height;
    
    for &worker_time in worker_times {
        let worker_time = worker_time as i64;
        let mut l_height: i64 = 0;
        let mut r_height: i64 = 1_000_000;
        
        while r_height >= l_height {
            let mid_height = (l_height + r_height) / 2;
            // Calculate time using the formula for sum of arithmetic sequence
            let use_time = mid_height * (worker_time + mid_height * worker_time) / 2;
            
            if use_time > target_time {
                r_height = mid_height - 1;
            } else {
                l_height = mid_height + 1;
            }
        }
        
        remaining_height -= r_height;
    }
    
    remaining_height <= 0
}

/// Finds the minimum number of seconds needed to remove the mountain
fn min_number_of_seconds(mountain_height: i32, worker_times: &[i32]) -> i64 {
    let mut l_time: i64 = 0;
    let mut r_time: i64 = 1_000_000_000_000_000_000; // 1e18
    
    while r_time >= l_time {
        let mid_time = (r_time + l_time) / 2;
        
        if cal_time(mountain_height as i64, worker_times, mid_time) {
            r_time = mid_time - 1;
        } else {
            l_time = mid_time + 1;
        }
    }
    
    r_time + 1
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the first line containing mountainHeight and workerTimesSize
    let first_line = lines.next().unwrap().unwrap();
    let mut iter = first_line.split_whitespace();
    let mountain_height: i32 = iter.next().unwrap().parse().unwrap();
    let worker_times_size: usize = iter.next().unwrap().parse().unwrap();
    
    // Read the worker times
    let second_line = lines.next().unwrap().unwrap();
    let worker_times: Vec<i32> = second_line
        .split_whitespace()
        .take(worker_times_size)
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Calculate and print the result
    println!("{}", min_number_of_seconds(mountain_height, &worker_times));
}