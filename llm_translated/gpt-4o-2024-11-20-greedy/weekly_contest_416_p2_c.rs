use std::io::{self, BufRead};
use std::cmp::Ordering;

fn cal_time(mountain_height: i64, worker_times: &[i32], target_time: i64) -> bool {
    let mut remaining_height = mountain_height;

    for &worker_time in worker_times {
        let mut l_height = 0_i64;
        let mut r_height = 1_000_000_i64;

        while l_height <= r_height {
            let mid_height = (l_height + r_height) / 2;
            let use_time = mid_height * (worker_time as i64 + mid_height * worker_time as i64) / 2;

            if use_time > target_time {
                r_height = mid_height - 1;
            } else {
                l_height = mid_height + 1;
            }
        }

        remaining_height -= r_height;

        if remaining_height <= 0 {
            return true;
        }
    }

    false
}

fn min_number_of_seconds(mountain_height: i64, worker_times: &[i32]) -> i64 {
    let mut l_time = 0_i64;
    let mut r_time = 1_000_000_000_000_000_000_i64;

    while l_time <= r_time {
        let mid_time = (l_time + r_time) / 2;

        if cal_time(mountain_height, worker_times, mid_time) {
            r_time = mid_time - 1;
        } else {
            l_time = mid_time + 1;
        }
    }

    l_time
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the mountain height and number of workers
    let first_line = lines.next().unwrap().unwrap();
    let mut inputs = first_line.split_whitespace();
    let mountain_height: i64 = inputs.next().unwrap().parse().unwrap();
    let worker_count: usize = inputs.next().unwrap().parse().unwrap();

    // Read worker times
    let second_line = lines.next().unwrap().unwrap();
    let worker_times: Vec<i32> = second_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    assert_eq!(worker_times.len(), worker_count);

    // Compute the minimum number of seconds
    let result = min_number_of_seconds(mountain_height, &worker_times);

    // Output the result
    println!("{}", result);
}