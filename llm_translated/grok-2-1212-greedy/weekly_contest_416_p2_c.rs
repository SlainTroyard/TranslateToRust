use std::io::{self, BufRead};

fn cal_time(mountain_height: i64, worker_times: &[i32], target_time: i64) -> bool {
    let mut mountain_height = mountain_height;
    for &worker_time in worker_times {
        let mut l_height = 0;
        let mut r_height = 1_000_000;
        while r_height >= l_height {
            let mid_height = (l_height + r_height) / 2;
            let use_time = mid_height * (worker_time as i64 + mid_height * worker_time as i64) / 2;
            if use_time > target_time {
                r_height = mid_height - 1;
            } else {
                l_height = mid_height + 1;
            }
        }
        mountain_height -= r_height;
    }
    mountain_height <= 0
}

fn min_number_of_seconds(mountain_height: i32, worker_times: &[i32]) -> i64 {
    let mut r_time = 1_000_000_000_000_000_000;
    let mut l_time = 0;
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

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read mountain height and worker times size
    let first_line = lines.next().unwrap()?;
    let mut iter = first_line.split_whitespace();
    let mountain_height: i32 = iter.next().unwrap().parse().unwrap();
    let worker_times_size: usize = iter.next().unwrap().parse().unwrap();

    // Read worker times
    let second_line = lines.next().unwrap()?;
    let worker_times: Vec<i32> = second_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Calculate and print result
    let result = min_number_of_seconds(mountain_height, &worker_times);
    println!("{}", result);

    Ok(())
}