use priority_queue::MinHeap;

mod solution {
    use super::MinHeap;

    pub fn min_number_of_seconds(mountain_height: i64, worker_times: Vec<i64>) -> i64 {
        let mut heap = MinHeap::new();
        for t in worker_times {
            heap.push((t, t, t));
        }
        let mut ans = 0;
        for _ in 0..mountain_height {
            let (nxt, delta, base) = heap.pop().expect("Heap should not be empty");
            ans = nxt;
            let new_next = nxt + delta + base;
            let new_delta = delta + base;
            heap.push((new_next, new_delta, base));
        }
        ans
    }
}

fn main() {
    use std::io;
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read first line");
    let mut parts = input.trim().split_whitespace();
    let mountain_height: i64 = parts.next().expect("Missing mountain height").parse().expect("Invalid mountain height");
    let worker_times_size: usize = parts.next().expect("Missing worker times size").parse().expect("Invalid worker times size");

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read worker times");
    let worker_times: Vec<i64> = input
        .trim()
        .split_whitespace()
        .take(worker_times_size)
        .map(|s| s.parse().expect("Invalid worker time"))
        .collect();

    let ans = solution::min_number_of_seconds(mountain_height, worker_times);
    println!("{}", ans);
}