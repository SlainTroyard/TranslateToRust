use std::io::{self, BufRead};

#[derive(Clone, Copy)]
struct DpNode {
    max: [i64; 4],
    idx: [u64; 4],
}

struct PriorityQueue {
    arr: Vec<i64>,
    size: usize,
}

impl PriorityQueue {
    fn new() -> Self {
        PriorityQueue {
            arr: Vec::new(),
            size: 0,
        }
    }

    fn push(&mut self, value: i64) {
        self.arr.push(value);
        let mut son = self.size;
        self.size += 1;

        while son > 0 {
            let father = (son - 1) / 2;
            if self.arr[son] >= self.arr[father] {
                break;
            }
            self.arr.swap(son, father);
            son = father;
        }
    }

    fn pop(&mut self) {
        if self.size == 0 {
            return;
        }
        self.size -= 1;
        self.arr.swap(0, self.size);

        let mut father = 0;
        loop {
            let left = 2 * father + 1;
            let right = 2 * father + 2;
            let mut son = father;

            if left < self.size && self.arr[left] < self.arr[son] {
                son = left;
            }
            if right < self.size && self.arr[right] < self.arr[son] {
                son = right;
            }
            if son == father {
                break;
            }
            self.arr.swap(father, son);
            father = son;
        }
    }

    fn top(&self) -> Option<i64> {
        if self.size == 0 {
            None
        } else {
            Some(self.arr[0])
        }
    }

    fn size(&self) -> usize {
        self.size
    }
}

fn merge_long(high: i32, low: i32) -> i64 {
    (high as i64) << 32 | (low as i64 & 0xFFFFFFFF)
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
        (idx >> 32 & 0xFFFF) as u16,
        (idx >> 16 & 0xFFFF) as u16,
        (idx & 0xFFFF) as u16,
    ];
    let i = i as u16;
    values[count] = i;
    let mut x = count as i32 - 1;
    while x >= 0 {
        let x_usize = x as usize;
        if values[x_usize] > i {
            values[x_usize + 1] = values[x_usize];
            values[x_usize] = i;
        } else {
            break;
        }
        x -= 1;
    }
    (values[0] as u64) << 48
        | (values[1] as u64) << 32
        | (values[2] as u64) << 16
        | values[3] as u64
}

fn maximum_weight(intervals: &[Vec<i32>], return_size: &mut usize) -> Vec<i32> {
    let n = intervals.len();
    let mut left_queue = PriorityQueue::new();
    let mut right_queue = PriorityQueue::new();

    for (i, interval) in intervals.iter().enumerate() {
        left_queue.push(merge_long(interval[0], i as i32));
        right_queue.push(merge_long(interval[1], i as i32));
    }

    let mut dp = vec![
        DpNode {
            max: [0; 4],
            idx: [0; 4],
        };
        n
    ];
    let mut last = DpNode {
        max: [0; 4],
        idx: [0; 4],
    };
    let mut max_weight = 0i64;
    let mut sel = u64::MAX;

    *return_size = 0;

    while left_queue.size() > 0 {
        let current = left_queue.top().unwrap();
        let i = lower_int(current) as usize;
        let edge = higher_int(current);

        while right_queue.size() > 0 {
            let right_top = right_queue.top().unwrap();
            let end = higher_int(right_top);
            if end >= edge {
                break;
            }
            let j = lower_int(right_top) as usize;

            for k in 0..3 {
                if last.max[k] < dp[j].max[k] || (last.max[k] == dp[j].max[k] && last.idx[k] > dp[j].idx[k]) {
                    last.max[k] = dp[j].max[k];
                    last.idx[k] = dp[j].idx[k];
                }
            }

            right_queue.pop();
        }

        dp[i] = last;

        for k in 0..4 {
            let prev = if k == 0 { 0 } else { last.max[k - 1] };
            if k == 0 || (k > 0 && prev > 0) {
                let weight = intervals[i][2] as i64;
                let current_sum = prev + weight;

                let idx = if k == 0 {
                    (i as u64) << 48
                } else {
                    insert_index(last.idx[k - 1], i as i32, k)
                };

                if dp[i].max[k] < current_sum || (dp[i].max[k] == current_sum && dp[i].idx[k] > idx) {
                    dp[i].max[k] = current_sum;
                    dp[i].idx[k] = idx;
                }

                if max_weight < dp[i].max[k] || (max_weight == dp[i].max[k] && sel > idx) {
                    *return_size = k + 1;
                    max_weight = dp[i].max[k];
                    sel = idx;
                }
            }
        }

        left_queue.pop();
    }

    let mut result = Vec::with_capacity(*return_size);
    for i in 0..*return_size {
        let shift = (3 - i) * 16;
        let val = (sel >> shift) & 0xFFFF;
        result.push(val as i32);
    }

    result
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap()).filter(|l| !l.is_empty());
    let n: usize = lines.next().unwrap().trim().parse().unwrap();

    let mut intervals = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap();
        let parts: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        intervals.push(vec![parts[0], parts[1], parts[2]]);
    }

    let mut return_size = 0;
    let result = maximum_weight(&intervals, &mut return_size);

    for val in result {
        print!("{} ", val);
    }
    println!();
}