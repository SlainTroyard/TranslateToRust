use std::cmp::Ordering;
use std::io::{self, BufRead, Write};

#[derive(Clone, Copy)]
struct DpNode {
    max: [i64; 4],
    idx: [u64; 4],
}

#[derive(Default)]
struct PriorityQueue {
    arr: Vec<i64>,
}

impl PriorityQueue {
    fn push(&mut self, value: i64) {
        let mut son = self.arr.len();
        self.arr.push(value);
        while let Some(father) = son.checked_sub(1).and_then(|x| x.checked_div(2)) {
            if self.arr[son] < self.arr[father] {
                self.arr.swap(son, father);
                son = father;
            } else {
                break;
            }
        }
    }

    fn pop(&mut self) -> Option<i64> {
        if self.arr.is_empty() {
            return None;
        }
        self.arr.swap(0, self.arr.len() - 1);
        let result = self.arr.pop();
        let mut father = 0;
        loop {
            let left = 2 * father + 1;
            let right = 2 * father + 2;
            let mut son = left;
            if left >= self.arr.len() {
                break;
            }
            if right < self.arr.len() && self.arr[left] > self.arr[right] {
                son = right;
            }
            if self.arr[father] <= self.arr[son] {
                break;
            }
            self.arr.swap(father, son);
            father = son;
        }
        result
    }
}

fn insert_index(idx: u64, i: usize, count: usize) -> u64 {
    let mut value = [0; 4];
    value[0] = (idx >> 48) as usize;
    value[1] = ((idx >> 32) & 0xFFFF) as usize;
    value[2] = ((idx >> 16) & 0xFFFF) as usize;
    value[count] = i;
    for x in (0..count).rev() {
        if value[x] > i {
            value[x + 1] = value[x];
            value[x] = i;
        } else {
            break;
        }
    }
    (value[0] as u64) << 48 | (value[1] as u64) << 32 | (value[2] as u64) << 16 | (value[3] as u64)
}

fn maximum_weight(intervals: &Vec<Vec<i32>>, return_size: &mut usize) -> Vec<usize> {
    let n = intervals.len();
    let mut dp = vec![DpNode { max: [0; 4], idx: [0; 4] }; n];
    let mut left_queue = PriorityQueue::default();
    let mut right_queue = PriorityQueue::default();

    for (i, interval) in intervals.iter().enumerate() {
        left_queue.push(((interval[0] as i64) << 32) | i as i64);
        right_queue.push(((interval[1] as i64) << 32) | i as i64);
    }

    let mut last = DpNode { max: [0; 4], idx: [0; 4] };

    while let Some(top) = left_queue.pop() {
        let i = top as usize & 0xFFFFFFFF;
        let edge = (top >> 32) as i32;

        while let Some(right_top) = right_queue.pop() {
            let j = right_top as usize & 0xFFFFFFFF;
            if (right_top >> 32) as i32 > edge {
                right_queue.push(right_top);
                break;
            }
            for k in 0..3 {
                if last.max[k] < dp[j].max[k] || (last.max[k] == dp[j].max[k] && last.idx[k] > dp[j].idx[k]) {
                    last.max[k] = dp[j].max[k];
                    last.idx[k] = dp[j].idx[k];
                }
            }
        }

        dp[i] = last;
        for k in 0..4 {
            let prev = if k == 0 { 0 } else { last.max[k - 1] };
            if k == 0 || (k > 0 && prev > 0) {
                let idx = if k == 0 { (i as u64) << 48 } else { insert_index(last.idx[k - 1], i, k) };
                let new_max = intervals[i][2] as i64 + prev;
                if dp[i].max[k] < new_max || (dp[i].max[k] == new_max && dp[i].idx[k] > idx) {
                    dp[i].max[k] = new_max;
                    dp[i].idx[k] = idx;
                }
                if dp[i].max[k] > last.max[*return_size - 1] || (dp[i].max[k] == last.max[*return_size - 1] && dp[i].idx[k] < last.idx[*return_size - 1]) {
                    *return_size = k + 1;
                    last.max[*return_size - 1] = dp[i].max[k];
                    last.idx[*return_size - 1] = dp[i].idx[k];
                }
            }
        }
    }

    let mut result = Vec::new();
    for i in 0..*return_size {
        result.push((last.idx[*return_size - 1] >> (3 - i) * 16) as usize & 0xFFFF);
    }
    result
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buf = String::new();

    stdin.read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();
    buf.clear();

    let mut intervals = Vec::with_capacity(n);
    for _ in 0..n {
        stdin.read_line(&mut buf).unwrap();
        let values: Vec<i32> = buf.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
        intervals.push(values);
        buf.clear();
    }

    let mut return_size = 0;
    let result = maximum_weight(&intervals, &mut return_size);

    for i in 0..return_size {
        write!(stdout, "{} ", result[i]).unwrap();
    }
    writeln!(stdout).unwrap();
}