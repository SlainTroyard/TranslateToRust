use std::io;

fn cal_time(mountain_height: i64, worker_times: &[i32], target_time: i64) -> bool {
    let mut current_mountain_height = mountain_height;
    let mut l_height: i64;
    let mut r_height: i64;
    let mut mid_height: i64;
    let mut use_time: i64;
    let max_height: i64 = 1_000_000; // Corresponds to 1e6 in C code

    for &worker_time in worker_times {
        l_height = 0;
        r_height = max_height;
        let worker_time_i64 = worker_time as i64;
        while r_height >= l_height {
            mid_height = (l_height + r_height) / 2;
            use_time = mid_height * worker_time_i64 * (mid_height + 1) / 2; // Correct formula from C's use_time calculation
            if use_time > target_time {
                r_height = mid_height - 1;
            } else {
                l_height = mid_height + 1;
            }
        }
        current_mountain_height -= r_height;
    }
    current_mountain_height <= 0
}

fn min_number_of_seconds(mountain_height: i32, worker_times: &[i32]) -> i64 {
    let mut r_time: i64 = 1_000_000_000_000_000_000; // Corresponds to 1e18 in C code
    let mut l_time: i64 = 0;
    let mut mid_time: i64;

    let mountain_height_i64 = mountain_height as i64;

    while r_time >= l_time {
        mid_time = (r_time + l_time) / 2;
        if cal_time(mountain_height_i64, worker_times, mid_time) {
            r_time = mid_time - 1;
        } else {
            l_time = mid_time + 1;
        }
    }
    r_time + 1
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let mut iter = input_line.trim().split_whitespace();
    let mountain_height: i32 = iter.next().unwrap().parse().unwrap();
    let worker_times_size: usize = iter.next().unwrap().parse().unwrap();

    let mut worker_times: Vec<i32> = Vec::with_capacity(worker_times_size);
    for _ in 0..worker_times_size {
        input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let worker_time: i32 = input_line.trim().parse().unwrap();
        worker_times.push(worker_time);
    }

    println!("{}", min_number_of_seconds(mountain_height, &worker_times));
}