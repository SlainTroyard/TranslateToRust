// Problem: Weekly Contest 416 Problem 2
// Translated from the provided C code into Rust, preserving logic and I/O format.
//
// The original C code reads two integers (mountainHeight, workerTimesSize), then
// reads workerTimesSize more integers (workerTimes), and outputs the result of
// the computation on a single line.
//
// We do the same here in Rust, using idiomatic Rust and basic error handling.

use std::io::{self, Read};

/// Determines whether the given target_time is sufficient for all workers to
/// remove mountain_height units of mountain, based on their respective worker_times.
fn cal_time(mut mountain_height: i64, worker_times: &[i64], target_time: i64) -> bool {
    // Each worker tries to remove as much as possible with a binary search
    // for the maximum height they can remove within target_time.
    for &time in worker_times {
        let mut l_height = 0i64;
        let mut r_height = 1_000_000i64; // 1e6 in C code
        while r_height >= l_height {
            let mid_height = (l_height + r_height) / 2;
            let use_time = mid_height * (time + mid_height * time) / 2;
            if use_time > target_time {
                r_height = mid_height - 1;
            } else {
                l_height = mid_height + 1;
            }
        }
        // After this binary search loop, r_height is the maximum height
        // the worker can remove within target_time. Subtract it from the mountain.
        mountain_height -= r_height;
    }
    // If we still have mountain left (mountain_height > 0), we fail (return false).
    // Otherwise, success (return true).
    mountain_height <= 0
}

/// Computes the minimum number of seconds needed for the workers to remove
/// mountain_height units of mountain, using binary search on time.
fn min_number_of_seconds(mountain_height: i64, worker_times: &[i64]) -> i64 {
    let mut l_time = 0i64;
    let mut r_time = 1_000_000_000_000_000_000i64; // 1e18 in C code
    while r_time >= l_time {
        let mid_time = (l_time + r_time) / 2;
        if cal_time(mountain_height, worker_times, mid_time) {
            r_time = mid_time - 1;
        } else {
            l_time = mid_time + 1;
        }
    }
    // r_time+1 is the smallest time that satisfies cal_time
    r_time + 1
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read all input from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Split input into tokens (similar to repeated scanf calls in C)
    let tokens: Vec<&str> = input.split_whitespace().collect();

    // Parse the first two integers: mountainHeight and workerTimesSize
    let mountain_height: i64 = tokens
        .get(0)
        .ok_or("Missing mountainHeight")?
        .parse()
        .map_err(|_| "Failed to parse mountainHeight")?;
    let worker_times_size: usize = tokens
        .get(1)
        .ok_or("Missing workerTimesSize")?
        .parse()
        .map_err(|_| "Failed to parse workerTimesSize")?;

    // Parse the next worker_times_size integers into a vector
    let mut worker_times: Vec<i64> = Vec::with_capacity(worker_times_size);
    for i in 0..worker_times_size {
        let value: i64 = tokens
            .get(2 + i)
            .ok_or("Not enough workerTimes provided")?
            .parse()
            .map_err(|_| "Failed to parse workerTime")?;
        worker_times.push(value);
    }

    // Compute and print the result, exactly one line with the answer
    let answer = min_number_of_seconds(mountain_height, &worker_times);
    println!("{}", answer);

    Ok(())
}