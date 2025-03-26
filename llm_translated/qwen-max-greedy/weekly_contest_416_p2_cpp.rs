use std::cmp::Ordering;
use std::io::{self, BufRead, Write};

#[derive(Eq, PartialEq)]
struct WorkerState {
    nxt: i64,
    delta: i64,
    base: i64,
}

impl Ord for WorkerState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.nxt.cmp(&other.nxt)
    }
}

impl PartialOrd for WorkerState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Solution;

impl Solution {
    pub fn min_number_of_seconds(mountain_height: i32, worker_times: Vec<i32>) -> i64 {
        use std::collections::BinaryHeap;

        let mut pq: BinaryHeap<WorkerState> = BinaryHeap::new();
        for t in worker_times {
            pq.push(WorkerState {
                nxt: t as i64,
                delta: t as i64,
                base: t as i64,
            });
        }

        let mut ans = 0;
        for _ in 0..mountain_height {
            if let Some(state) = pq.pop() {
                ans = state.nxt;
                pq.push(WorkerState {
                    nxt: state.nxt + state.delta + state.base,
                    delta: state.delta + state.base,
                    base: state.base,
                });
            }
        }
        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut stdin_lock = stdin.lock();
    let mut stdout_lock = stdout.lock();

    let mut input = String::new();
    stdin_lock.read_line(&mut input).unwrap();
    let input: Vec<&str> = input.trim().split_whitespace().collect();
    let mountain_height: i32 = input[0].parse().unwrap();
    let worker_times_size: usize = input[1].parse().unwrap();

    let mut worker_times = Vec::with_capacity(worker_times_size);
    for _ in 0..worker_times_size {
        input.clear();
        stdin_lock.read_line(&mut input).unwrap();
        worker_times.push(input.trim().parse().unwrap());
    }

    let s = Solution;
    let result = s.min_number_of_seconds(mountain_height, worker_times);
    writeln!(stdout_lock, "{}", result).unwrap();
}