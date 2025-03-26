use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    // Read the mountain height and the number of worker times
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut iter = input.split_whitespace();
    let mountain_height: i32 = iter.next().expect("Missing mountain height").parse().expect("Invalid mountain height");
    let worker_times_size: usize = iter.next().expect("Missing worker times size").parse().expect("Invalid worker times size");

    // Read the worker times
    let mut worker_times = Vec::with_capacity(worker_times_size);
    for _ in 0..worker_times_size {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");
        let time: i32 = input.trim().parse().expect("Invalid worker time");
        worker_times.push(time);
    }

    // Calculate the minimum number of seconds
    let result = min_number_of_seconds(mountain_height, &worker_times);
    println!("{}", result);
}

fn min_number_of_seconds(mountain_height: i32, worker_times: &[i32]) -> i64 {
    // Use a min-heap (priority queue) to keep track of the next available time for each worker
    let mut pq = BinaryHeap::new();
    for &t in worker_times {
        pq.push(Reverse((t as i64, t as i64, t)));
    }

    let mut ans = 0;
    for _ in 0..mountain_height {
        if let Some(Reverse((nxt, delta, base))) = pq.pop() {
            ans = nxt;
            pq.push(Reverse((nxt + delta + base, delta + base, base)));
        }
    }

    ans
}