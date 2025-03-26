use std::io;

fn cal_time(mountain_height: i64, worker_times: &[i32], target_time: i64) -> bool {
    let mut l_height = 0;
    let mut r_height = 1_000_000;
    let mut mid_height = 0;
    let mut use_time = 0;

    for &worker_time in worker_times {
        l_height = 0;
        r_height = 1_000_000;
        while r_height >= l_height {
            mid_height = (l_height + r_height) / 2;
            use_time = mid_height * (worker_time as i64 + mid_height * worker_time as i64) / 2;
            if use_time > target_time {
                r_height = mid_height - 1;
            } else {
                l_height = mid_height + 1;
            }
        }
        mountain_height -= r_height as i64;
    }
    mountain_height <= 0
}

fn min_number_of_seconds(mountain_height: i64, worker_times: &[i32]) -> i64 {
    let mut r_time = 1_000_000_000_000_000_000;
    let mut l_time = 0;
    let mut mid_time = 0;

    while r_time >= l_time {
        mid_time = (r_time + l_time) / 2;
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
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let mountain_height: i64 = iter.next().unwrap().parse().expect("Invalid input");
    let worker_times_size: usize = iter.next().unwrap().parse().expect("Invalid input");

    let mut worker_times = vec![0; worker_times_size];
    for i in 0..worker_times_size {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        worker_times[i] = input.trim().parse().expect("Invalid input");
    }

    let result = min_number_of_seconds(mountain_height, &worker_times);
    println!("{}", result);
}