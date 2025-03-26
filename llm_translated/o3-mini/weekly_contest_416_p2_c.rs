use std::io::{self, BufRead};

// This function checks whether the given target time is enough to complete or exceed the mountainHeight.
// For each worker time, we do a binary search to determine the maximum height the worker can work on within target_time.
// Then, we subtract that height from the remaining mountainHeight.
fn cal_time(mut mountain_height: i64, worker_times: &[i32], target_time: i64) -> bool {
    // Iterate over each worker's time (as provided in the input)
    for &worker in worker_times {
        // The binary search for the maximum height that this worker can build within target_time.
        // The worker builds at a rate that causes total time = (height * (worker + worker*height)) / 2.
        // We know that from the formula in the C code.
        let mut l_height: i64 = 0;
        let mut r_height: i64 = 1_000_000;
        let mut r_valid: i64 = 0; // will store the valid maximum height
        
        // Binary search: while left is <= right
        while l_height <= r_height {
            let mid_height = (l_height + r_height) / 2;
            // Compute use_time as mid * (worker + mid * worker) / 2,
            // which is equivalent to: worker * mid * (mid + 1) / 2.
            // We perform calculations using i64 to avoid overflow.
            let use_time = mid_height * (worker as i64 + mid_height * worker as i64) / 2;
            if use_time > target_time {
                // If time needed exceeds target_time, reduce the search space.
                r_height = mid_height - 1;
            } else {
                // Otherwise, record this as a valid height and search for a bigger one.
                r_valid = mid_height;
                l_height = mid_height + 1;
            }
        }
        // Subtract the maximum height produced by the current worker from mountain_height.
        mountain_height -= r_valid;
    }
    // If mountain_height is reduced to 0 or lower, the target_time is enough.
    mountain_height <= 0
}

// This function performs binary search on the time needed.
// It returns the minimum number of seconds required such that the workers can achieve the mountainHeight.
fn min_number_of_seconds(mountain_height: i32, worker_times: &[i32]) -> i64 {
    // Define lower and upper bounds for binary search.
    let mut l_time: i64 = 0;
    let mut r_time: i64 = 1_000_000_000_000_000_000; // 1e18
    let mut ans: i64 = 0;
    
    // Binary search loop.
    while l_time <= r_time {
        let mid_time = (l_time + r_time) / 2;
        // If current mid_time can finish building the mountain, try a smaller time.
        if cal_time(mountain_height as i64, worker_times, mid_time) {
            ans = mid_time;
            r_time = mid_time - 1;
        } else {
            // Otherwise, try a larger time.
            l_time = mid_time + 1;
        }
    }
    // In the original C code, the result is ++r_time but due to the binary search logic,
    // Here we directly use ans as the minimal sufficient time.
    ans
}

fn main() -> io::Result<()> {
    // Create a buffered reader to read from stdin.
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut input_line = String::new();

    // Read the first line containing mountainHeight and workerTimesSize.
    reader.read_line(&mut input_line)?;
    let mut parts = input_line.split_whitespace();
    let mountain_height: i32 = parts
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Missing mountain height"))?
        .parse()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid mountain height"))?;
    let worker_times_size: usize = parts
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Missing worker times size"))?
        .parse()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid worker times size"))?;

    // Read the next lines to get workerTimes values.
    // We accumulate tokens from lines until we have worker_times_size numbers.
    let mut worker_times: Vec<i32> = Vec::with_capacity(worker_times_size);
    while worker_times.len() < worker_times_size {
        input_line.clear();
        reader.read_line(&mut input_line)?;
        for token in input_line.split_whitespace() {
            if worker_times.len() < worker_times_size {
                let value: i32 = token.parse().map_err(|_| {
                    io::Error::new(io::ErrorKind::InvalidInput, "Invalid worker time value")
                })?;
                worker_times.push(value);
            }
        }
    }

    // Calculate the minimal number of seconds required.
    let result = min_number_of_seconds(mountain_height, &worker_times);
    // Print the result exactly as "%lld\n" in C.
    println!("{}", result);
    Ok(())
}