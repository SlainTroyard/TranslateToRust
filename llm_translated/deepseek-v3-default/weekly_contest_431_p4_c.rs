use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io;

#[derive(Clone, Copy, PartialEq, Eq)]
struct DpNode {
    max: [i64; 4],
    idx: [u64; 4],
}

impl Default for DpNode {
    fn default() -> Self {
        DpNode {
            max: [0; 4],
            idx: [0; 4],
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Interval {
    start: i32,
    end: i32,
    weight: i32,
}

impl Ord for Interval {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start.cmp(&other.start)
    }
}

impl PartialOrd for Interval {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn father_node(i: usize) -> usize {
    if i == 0 { usize::MAX } else { (i - 1) >> 1 }
}

fn left_node(i: usize) -> usize {
    (i << 1) + 1
}

fn right_node(i: usize) -> usize {
    (i << 1) + 2
}

fn higher_int(value: i64) -> i32 {
    (value >> 32) as i32
}

fn lower_int(value: i64) -> usize {
    (value & 0xFFFFFFFF) as usize
}

fn merge_long(high: i32, low: usize) -> i64 {
    ((high as i64) << 32) | (low as i64)
}

fn insert_index(idx: u64, i: usize, count: usize) -> u64 {
    let mut value = [
        (idx >> 48) as usize,
        ((idx >> 32) & 0xFFFF) as usize,
        ((idx >> 16) & 0xFFFF) as usize,
        i,
    ];
    for x in (0..count).rev() {
        if value[x] > i {
            value[x + 1] = value[x];
            value[x] = i;
        }
    }
    ((value[0] as u64) << 48)
        | ((value[1] as u64) << 32)
        | ((value[2] as u64) << 16)
        | (value[3] as u64)
}

fn maximum_weight(intervals: &[Interval], return_size: &mut usize) -> Vec<usize> {
    let n = intervals.len();
    let mut dp = vec![DpNode::default(); n];
    let mut last = DpNode::default();
    let mut left_queue = BinaryHeap::new();
    let mut right_queue = BinaryHeap::new();

    for (i, interval) in intervals.iter().enumerate() {
        left_queue.push((interval.start, i));
        right_queue.push((interval.end, i));
    }

    let mut max = 0;
    let mut sel = u64::MAX;

    while let Some((edge, i)) = left_queue.pop() {
        while let Some(&(end, j)) = right_queue.peek() {
            if edge > end {
                right_queue.pop();
                for k in 0..3 {
                    if last.max[k] < dp[j].max[k] || (last.max[k] == dp[j].max[k] && last.idx[k] > dp[j].idx[k]) {
                        last.max[k] = dp[j].max[k];
                        last.idx[k] = dp[j].idx[k];
                    }
                }
            } else {
                break;
            }
        }

        dp[i] = last;
        for k in 0..4 {
            let prev = if k == 0 { 0 } else { last.max[k - 1] };
            if k == 0 || (k > 0 && prev > 0) {
                let idx = if k == 0 {
                    (i as u64) << 48
                } else {
                    insert_index(last.idx[k - 1], i, k)
                };
                if dp[i].max[k] < intervals[i].weight as i64 + prev
                    || (dp[i].max[k] == intervals[i].weight as i64 + prev && dp[i].idx[k] > idx)
                {
                    dp[i].max[k] = intervals[i].weight as i64 + prev;
                    dp[i].idx[k] = idx;
                }
                if max < dp[i].max[k] || (max == dp[i].max[k] && sel > dp[i].idx[k]) {
                    *return_size = k + 1;
                    max = dp[i].max[k];
                    sel = dp[i].idx[k];
                }
            }
        }
    }

    let mut result = Vec::with_capacity(*return_size);
    for i in 0..*return_size {
        result.push((sel >> (3 - i << 4)) & 0xFFFF);
    }
    result
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut intervals = Vec::with_capacity(n);
    for _ in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<i32> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        intervals.push(Interval {
            start: parts[0],
            end: parts[1],
            weight: parts[2],
        });
    }

    let mut return_size = 0;
    let result = maximum_weight(&intervals, &mut return_size);

    for i in 0..return_size {
        print!("{} ", result[i]);
    }
    println!();
}