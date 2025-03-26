use std::io::{self, Write};
use std::cmp::Ordering;

fn cal_time(mountain_height: i64, worker_times: &[i32], target_time: i64) -> bool {
    let mut mountain_height = mountain_height;
    for &worker_time in worker_times {
        let mut l_height: i64 = 0;
        let mut r_height: i64 = 1_000_000;
        let mut mid_height: i64;
        let mut use_time: i64;
        while r_height >= l_height {
            // Calculate the middle height
            mid_height = (l_height + r_height) / 2;
            
            // Calculate the time used by a single worker
            use_time = mid_height * (worker_time as i64 + mid_height * worker_time as i64) / 2;

            // If time used exceeds the target, reduce the height
            if use_time > target_time {
                r_height = mid_height - 1;
            } else {
                l_height = mid_height + 1;
            }
        }
        // Subtract the maximum height this worker can complete
        mountain_height -= r_height;
    }
    // Check if the mountain can be leveled
    mountain_height <= 0
}

fn min_number_of_seconds(mountain_height: i64, worker_times: &[i32]) -> i64 {
    let mut l_time: i64 = 0;
    let mut r_time: i64 = 1_000_000_000_000_000_000; // 1e18
    let mut mid_time: i64;

    while r_time >= l_time {
        // Calculate the middle time
        mid_time = (r_time + l_time) / 2;

        // Check if it's possible to level the mountain in `mid_time`
        if cal_time(mountain_height, worker_times, mid_time) {
            // Try a smaller time
            r_time = mid_time - 1;
        } else {
            // Try a larger time
            l_time = mid_time + 1;
        }
    }

    // Return the first time that works
    r_time + 1
}

fn main() {
    let mut input = String::new();

    // Read the input for mountain_height and workerTimesSize
    io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.trim().split_whitespace();
    let mountain_height: i64 = parts.next().unwrap().parse().expect("Invalid mountain height");
    let worker_times_size: usize = parts.next().unwrap().parse().expect("Invalid workerTimesSize");

    // Read the input for worker_times
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let worker_times: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid worker time"))
        .collect();

    // Ensure the number of worker times matches the size
    assert_eq!(worker_times.len(), worker_times_size);

    // Calculate the minimum number of seconds and print the result
    let result = min_number_of_seconds(mountain_height, &worker_times);
    println!("{}", result);
}