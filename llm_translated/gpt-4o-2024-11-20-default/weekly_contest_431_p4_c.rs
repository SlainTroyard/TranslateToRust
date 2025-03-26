use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io;
use std::usize;

const INVALID_INDEX: u64 = u64::MAX;

#[derive(Clone, Debug, Default)]
struct DpNode {
    max: [i64; 4],
    idx: [u64; 4],
}

#[derive(Default)]
struct PriorityQueue {
    heap: BinaryHeap<i64>,
}

impl PriorityQueue {
    fn push(&mut self, value: i64) {
        self.heap.push(-value); // Use negative to emulate a min-heap
    }

    fn pop(&mut self) -> Option<i64> {
        self.heap.pop().map(|value| -value) // Revert the negativity
    }

    fn peek(&self) -> Option<i64> {
        self.heap.peek().map(|value| -*value)
    }

    fn len(&self) -> usize {
        self.heap.len()
    }
}

fn merge_long(high: i32, low: i32) -> i64 {
    ((high as i64) << 32) | (low as i64 & 0xFFFFFFFF)
}

fn higher_int(val: i64) -> i32 {
    (val >> 32) as i32
}

fn lower_int(val: i64) -> i32 {
    val as i32 & 0xFFFFFFFF
}

fn insert_index(idx: u64, i: i32, count: usize) -> u64 {
    let mut values = [INVALID_INDEX; 4];
    // Decode the previous indices
    values[0] = idx >> 48;
    values[1] = (idx >> 32) & 0xFFFF;
    values[2] = (idx >> 16) & 0xFFFF;
    values[count] = i as u64;

    // Insert the new index in order
    let mut x = count as isize - 1;
    while x >= 0 && values[x as usize] > i as u64 {
        values[(x + 1) as usize] = values[x as usize];
        values[x as usize] = i as u64;
        x -= 1;
    }

    // Encode the updated indices into a single u64
    (values[0] << 48) | (values[1] << 32) | (values[2] << 16) | values[3]
}

fn maximum_weight(
    intervals: Vec<[i32; 3]>,
    intervals_size: usize,
) -> Vec<i32> {
    let mut left_queue = PriorityQueue::default();
    let mut right_queue = PriorityQueue::default();
    let mut max = 0;
    let mut sel = INVALID_INDEX;

    let mut dp = vec![DpNode::default(); intervals_size];
    let mut last = DpNode::default();
    let mut result: Vec<i32> = Vec::new();

    for i in 0..intervals_size {
        left_queue.push(merge_long(intervals[i][0], i as i32));
        right_queue.push(merge_long(intervals[i][1], i as i32));
    }

    while left_queue.len() > 0 {
        let i = lower_int(left_queue.peek().unwrap());
        let edge = higher_int(left_queue.peek().unwrap());
        while right_queue.len() > 0 && edge > higher_int(right_queue.peek().unwrap()) {
            let j = lower_int(right_queue.peek().unwrap());
            for k in 0..3 {
                if last.max[k] < dp[j as usize].max[k]
                    || (last.max[k] == dp[j as usize].max[k]
                        && last.idx[k] > dp[j as usize].idx[k])
                {
                    last.max[k] = dp[j as usize].max[k];
                    last.idx[k] = dp[j as usize].idx[k];
                }
            }
            right_queue.pop();
        }

        dp[i as usize] = last.clone();
        for k in 0..4 {
            let prev = if k == 0 { 0 } else { last.max[k - 1] };

            if k == 0 || (k > 0 && prev > 0) {
                let idx = if k == 0 {
                    (i as u64) << 48
                } else {
                    insert_index(last.idx[k - 1], i, k)
                };

                let new_val = intervals[i as usize][2] as i64 + prev;
                if dp[i as usize].max[k] < new_val
                    || (dp[i as usize].max[k] == new_val && dp[i as usize].idx[k] > idx)
                {
                    dp[i as usize].max[k] = new_val;
                    dp[i as usize].idx[k] = idx;
                }

                if max < dp[i as usize].max[k]
                    || (max == dp[i as usize].max[k] && sel > dp[i as usize].idx[k])
                {
                    max = dp[i as usize].max[k];
                    sel = dp[i as usize].idx[k];
                    result.clear();
                }
            }
        }

        left_queue.pop();
    }

    // Decode selected indices
    for i in 0..4 {
        let index = (sel >> (3 - i) * 16) & 0xFFFF;
        if index != INVALID_INDEX {
            result.push(index as i32);
        }
    }

    result
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n: usize = input.trim().parse().unwrap();
    let mut intervals = Vec::with_capacity(n);

    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let values: Vec<i32> = input
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        intervals.push([values[0], values[1], values[2]]);
    }

    let result = maximum_weight(intervals, n);

    for value in result {
        print!("{} ", value);
    }
    println!();
}