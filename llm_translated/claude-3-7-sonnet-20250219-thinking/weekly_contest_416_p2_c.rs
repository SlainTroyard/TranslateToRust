use std::io;

fn cal_time(mountain_height: i64, worker_times: &[i32], target_time: i64) -> bool {
    let mut remaining_height = mountain_height;
    
    for &worker_time in worker_times {
        let mut l_height = 0i64;
        let mut r_height = 1_000_000i64; // 1e6
        
        while r_height >= l_height {
            let mid_height = (l_height + r_height) / 2;
            let use_time = mid_height * (worker_time as i64 + mid_height * worker_time as i64) / 2;
            
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

fn min_number_of_seconds(mountain_height: i32, worker_times: &[i32]) -> i64 {
    let mut l_time = 0i64;
    let mut r_time = 1_000_000_000_000_000_000i64; // 1e18
    
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
    // Read mountain height and worker times size
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut parts = input.trim().split_whitespace();
    
    let mountain_height: i32 = parts.next()
        .expect("Missing mountain height")
        .parse()
        .expect("Invalid mountain height");
    
    let worker_times_size: usize = parts.next()
        .expect("Missing worker times size")
        .parse()
        .expect("Invalid worker times size");
    
    // Read worker times
    let mut worker_times = Vec::with_capacity(worker_times_size);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    for (i, part) in input.trim().split_whitespace().enumerate() {
        if i >= worker_times_size {
            break;
        }
        let worker_time: i32 = part.parse().expect("Invalid worker time");
        worker_times.push(worker_time);
    }
    
    // Calculate and print the result
    let result = min_number_of_seconds(mountain_height, &worker_times);
    println!("{}", result);
}