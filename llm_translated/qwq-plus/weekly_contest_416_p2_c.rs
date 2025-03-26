use std::io;

/// Calculates whether the given target_time is sufficient to reduce the mountain to zero or below.
///
/// # Arguments
/// * `mountain_height` - The initial height of the mountain.
/// * `worker_times` - Array of worker times (each worker's time per unit height).
/// * `target_time` - The target time to check.
///
/// # Returns
/// `true` if the mountain can be reduced to zero or below in the target_time, else `false`.
fn cal_time(mountain_height: i64, worker_times: &[i32], target_time: i64) -> bool {
    let mut current_height = mountain_height;
    for &worker_time in worker_times {
        let a = worker_time as i64;
        let mut l = 0i64;
        let mut r = 1_000_000i64; // 1e6 as per original code
        while r >= l {
            let mid = (l + r) / 2;
            let use_time = a * mid * (mid + 1) / 2;
            if use_time > target_time {
                r = mid - 1;
            } else {
                l = mid + 1;
            }
        }
        current_height -= r;
    }
    current_height <= 0
}

/// Finds the minimal time required to reduce the mountain to zero or below using binary search.
///
/// # Arguments
/// * `mountain_height` - The initial height of the mountain.
/// * `worker_times` - Array of worker times.
///
/// # Returns
/// The minimal number of seconds required.
fn min_number_of_seconds(mountain_height: i32, worker_times: &[i32]) -> i64 {
    let mut l_time = 0i64;
    let mut r_time = 1_000_000_000_000_000_000i64; // 1e18 as per original code
    while r_time >= l_time {
        let mid_time = (l_time + r_time) / 2;
        if cal_time(mountain_height as i64, worker_times, mid_time) {
            r_time = mid_time - 1;
        } else {
            l_time = mid_time + 1;
        }
    }
    r_time + 1 // Equivalent to C's ++r_time
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let mountain_height: i32 = iter.next().unwrap().parse().unwrap();
    let worker_times_size: usize = iter.next().unwrap().parse().unwrap();
    let worker_times: Vec<i32> = iter
        .map(|s| s.parse().unwrap())
        .take(worker_times_size)
        .collect();
    let result = min_number_of_seconds(mountain_height, &worker_times);
    println!("{}", result);
}