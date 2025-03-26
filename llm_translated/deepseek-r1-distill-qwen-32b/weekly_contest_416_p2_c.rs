use std::io;

// Function to calculate if the mountain can be reduced to zero or below within target_time
fn cal_time(mountain_height: i64, worker_times: &[i64], target_time: i64) -> bool {
    let mut remaining_height = mountain_height;
    for &time in worker_times {
        let mut l = 0;
        let mut r = 1_000_000;
        while l <= r {
            let mid = (l + r) / 2;
            let use_time = mid * (time + mid * time) / 2;
            if use_time > target_time {
                r = mid - 1;
            } else {
                l = mid + 1;
            }
        }
        remaining_height -= r;
        if remaining_height <= 0 {
            break;
        }
    }
    remaining_height <= 0
}

// Function to find the minimum number of seconds needed to reduce the mountain to zero
fn min_number_of_seconds(mountain_height: i64, worker_times: &[i64]) -> i64 {
    let mut l_time = 0;
    let mut r_time = 1_000_000_000_000_000_000; // 1e18
    while l_time <= r_time {
        let mid_time = (l_time + r_time) / 2;
        if cal_time(mountain_height, worker_times, mid_time) {
            r_time = mid_time - 1;
        } else {
            l_time = mid_time + 1;
        }
    }
    r_time + 1
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.trim().split_whitespace();
    let mountain_height: i64 = parts.next().unwrap().parse().unwrap();
    let worker_times_size: usize = parts.next().unwrap().parse().unwrap();

    let mut worker_times = Vec::with_capacity(worker_times_size);
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    for &time_str in parts.iter().take(worker_times_size) {
        worker_times.push(time_str.parse().unwrap());
    }

    let result = min_number_of_seconds(mountain_height, &worker_times);
    println!("{}", result);
}