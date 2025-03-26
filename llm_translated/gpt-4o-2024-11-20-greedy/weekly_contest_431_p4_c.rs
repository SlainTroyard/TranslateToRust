use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io::{self, BufRead};

#[derive(Debug, Clone)]
struct DpNode {
    max: [i64; 4],
    idx: [u64; 4],
}

#[derive(Debug)]
struct PriorityQueue {
    heap: BinaryHeap<i64>,
}

impl PriorityQueue {
    fn new() -> Self {
        PriorityQueue {
            heap: BinaryHeap::new(),
        }
    }

    fn push(&mut self, value: i64) {
        self.heap.push(-value); // Use negative values to simulate a min-heap
    }

    fn pop(&mut self) -> Option<i64> {
        self.heap.pop().map(|v| -v) // Convert back to positive values
    }

    fn peek(&self) -> Option<i64> {
        self.heap.peek().map(|v| -v) // Convert back to positive values
    }

    fn len(&self) -> usize {
        self.heap.len()
    }
}

fn merge_long(high: i32, low: i32) -> i64 {
    ((high as i64) << 32) | (low as i64 & 0xFFFFFFFF)
}

fn higher_int(value: i64) -> i32 {
    (value >> 32) as i32
}

fn lower_int(value: i64) -> i32 {
    (value & 0xFFFFFFFF) as i32
}

fn insert_index(idx: u64, i: i32, count: usize) -> u64 {
    let mut values = [
        (idx >> 48) as u16,
        ((idx >> 32) & 0xFFFF) as u16,
        ((idx >> 16) & 0xFFFF) as u16,
        0,
    ];
    values[count] = i as u16;

    for x in (0..count).rev() {
        if values[x] > i as u16 {
            values.swap(x, x + 1);
        } else {
            break;
        }
    }

    (values[0] as u64) << 48
        | (values[1] as u64) << 32
        | (values[2] as u64) << 16
        | (values[3] as u64)
}

fn maximum_weight(intervals: Vec<Vec<i32>>) -> Vec<i32> {
    let intervals_size = intervals.len();
    let mut left_queue = PriorityQueue::new();
    let mut right_queue = PriorityQueue::new();
    let mut dp = vec![DpNode {
        max: [0; 4],
        idx: [0; 4],
    }; intervals_size];
    let mut last = DpNode {
        max: [0; 4],
        idx: [0; 4],
    };

    for (i, interval) in intervals.iter().enumerate() {
        left_queue.push(merge_long(interval[0], i as i32));
        right_queue.push(merge_long(interval[1], i as i32));
    }

    let mut max = 0;
    let mut sel = u64::MAX;
    let mut return_size = 0;

    while left_queue.len() > 0 {
        let i = lower_int(left_queue.peek().unwrap()) as usize;
        let edge = higher_int(left_queue.peek().unwrap());

        while right_queue.len() > 0 && edge > higher_int(right_queue.peek().unwrap()) {
            let j = lower_int(right_queue.peek().unwrap()) as usize;
            for k in 0..3 {
                if last.max[k] < dp[j].max[k]
                    || (last.max[k] == dp[j].max[k] && last.idx[k] > dp[j].idx[k])
                {
                    last.max[k] = dp[j].max[k];
                    last.idx[k] = dp[j].idx[k];
                }
            }
            right_queue.pop();
        }

        dp[i] = last.clone();

        for k in 0..4 {
            let prev = if k == 0 { 0 } else { last.max[k - 1] };
            if k == 0 || (k > 0 && prev > 0) {
                let idx = if k == 0 {
                    (i as u64) << 48
                } else {
                    insert_index(last.idx[k - 1], i as i32, k)
                };

                if dp[i].max[k] < intervals[i][2] as i64 + prev
                    || (dp[i].max[k] == intervals[i][2] as i64 + prev && dp[i].idx[k] > idx)
                {
                    dp[i].max[k] = intervals[i][2] as i64 + prev;
                    dp[i].idx[k] = idx;
                }

                if max < dp[i].max[k] || (max == dp[i].max[k] && sel > dp[i].idx[k]) {
                    return_size = k + 1;
                    max = dp[i].max[k];
                    sel = dp[i].idx[k];
                }
            }
        }

        left_queue.pop();
    }

    let mut result = vec![0; return_size];
    for i in 0..return_size {
        result[i] = ((sel >> ((3 - i) * 16)) & 0xFFFF) as i32;
    }

    result
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut intervals = Vec::new();

    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let values: Vec<i32> = line
            .split_whitespace()
            .map(|v| v.parse().unwrap())
            .collect();
        intervals.push(values);
    }

    let result = maximum_weight(intervals);

    for (i, value) in result.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", value);
    }
    println!();
}