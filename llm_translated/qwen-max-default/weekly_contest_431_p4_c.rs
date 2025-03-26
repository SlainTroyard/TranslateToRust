use std::cmp::Ordering;
use std::io::{self, BufRead, Write};

#[derive(Clone, Copy)]
struct DpNode {
    max: [i64; 4],
    idx: [u64; 4],
}

#[derive(Debug)]
struct PriorityQueue {
    arr: Vec<i64>,
}

impl PriorityQueue {
    fn new() -> Self {
        PriorityQueue { arr: Vec::new() }
    }

    fn push(&mut self, value: i64) {
        let mut son = self.arr.len();
        self.arr.push(value);
        while son > 0 {
            let father = (son - 1) >> 1;
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
        while let Some(left) = LEFT_NODE(father).checked_sub(1) {
            let right = RIGHT_NODE(father).checked_sub(1);
            let son = match (right, &self.arr.get(left), &self.arr.get(right)) {
                (Some(r), Some(&l_val), Some(&r_val)) => {
                    if l_val < r_val {
                        left
                    } else {
                        r
                    }
                }
                _ => left,
            };
            if self.arr[son] < self.arr[father] {
                self.arr.swap(son, father);
                father = son;
            } else {
                break;
            }
        }
        result
    }

    fn peek(&self) -> Option<&i64> {
        self.arr.first()
    }
}

fn FATHER_NODE(i: usize) -> usize {
    if i == 0 { 0 } else { (i - 1) >> 1 }
}

fn LEFT_NODE(i: usize) -> usize {
    (i << 1) + 1
}

fn RIGHT_NODE(i: usize) -> usize {
    (i << 1) + 2
}

fn HIGHER_INT(i: i64) -> i32 {
    (i as u64 >> 32) as i32
}

fn LOWER_INT(i: i64) -> i32 {
    (i as u64 & 0xFFFFFFFF) as i32
}

fn MERGE_LONG(i: i32, j: i32) -> i64 {
    ((i as u64) << 32 | (j as u64)) as i64
}

fn insert_index(idx: u64, i: i32, count: i32) -> u64 {
    let mut value = [0; 4];
    value[0] = (idx >> 48) as i32;
    value[1] = (idx >> 32 & 0xFFFF) as i32;
    value[2] = (idx >> 16 & 0xFFFF) as i32;
    value[count as usize] = i;
    for x in (count - 1) as usize..0 {
        if value[x] > i {
            value[x + 1] = value[x];
            value[x] = i;
        } else {
            break;
        }
    }
    (value[0] as u64) << 48 | (value[1] as u64) << 32 | (value[2] as u64) << 16 | (value[3] as u64)
}

fn maximum_weight(intervals: &Vec<Vec<i32>>, return_size: &mut usize) -> Vec<i32> {
    let n = intervals.len();
    let mut last = DpNode { max: [0; 4], idx: [0; 4] };
    let mut dp = vec![DpNode { max: [0; 4], idx: [0; 4] }; n];
    let mut left_queue = PriorityQueue::new();
    let mut right_queue = PriorityQueue::new();

    for i in 0..n {
        left_queue.push(MERGE_LONG(intervals[i][0], i as i32));
        right_queue.push(MERGE_LONG(intervals[i][1], i as i32));
    }

    let mut max = 0;
    let mut sel = u64::MAX;

    while let Some(top) = left_queue.peek() {
        let i = LOWER_INT(*top) as usize;
        let edge = HIGHER_INT(*top);

        while let Some(right_top) = right_queue.peek() {
            if HIGHER_INT(*right_top) < edge {
                let j = LOWER_INT(*right_top) as usize;
                for k in 0..3 {
                    if last.max[k] < dp[j].max[k] || (last.max[k] == dp[j].max[k] && last.idx[k] > dp[j].idx[k]) {
                        last.max[k] = dp[j].max[k];
                        last.idx[k] = dp[j].idx[k];
                    }
                }
                right_queue.pop();
            } else {
                break;
            }
        }

        dp[i] = last;

        for k in 0..4 {
            let prev = if k == 0 { 0 } else { last.max[k - 1] };
            if k == 0 || (k > 0 && prev > 0) {
                let idx = if k == 0 { (i as u64) << 48 } else { insert_index(last.idx[k - 1], i as i32, k as i32) };
                let new_max = intervals[i][2] + prev;
                if dp[i].max[k] < new_max || (dp[i].max[k] == new_max && dp[i].idx[k] > idx) {
                    dp[i].max[k] = new_max;
                    dp[i].idx[k] = idx;
                }
                if max < dp[i].max[k] || (max == dp[i].max[k] && sel > dp[i].idx[k]) {
                    *return_size = k + 1;
                    max = dp[i].max[k];
                    sel = dp[i].idx[k];
                }
            }
        }

        left_queue.pop();
    }

    let mut result = Vec::with_capacity(*return_size);
    for i in 0..*return_size {
        result.push((sel >> (3 - i) * 16) as i32 & 0xFFFF);
    }
    result
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut stdin_lock = stdin.lock();
    let mut stdout_lock = stdout.lock();

    let mut input = String::new();
    stdin_lock.read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut intervals = Vec::with_capacity(n);
    for _ in 0..n {
        input.clear();
        stdin_lock.read_line(&mut input).unwrap();
        let mut iter = input.trim().split_whitespace();
        let start: i32 = iter.next().unwrap().parse().unwrap();
        let end: i32 = iter.next().unwrap().parse().unwrap();
        let weight: i32 = iter.next().unwrap().parse().unwrap();
        intervals.push(vec![start, end, weight]);
    }

    let mut return_size = 0;
    let result = maximum_weight(&intervals, &mut return_size);

    for i in 0..return_size {
        write!(stdout_lock, "{} ", result[i]).unwrap();
    }
    writeln!(stdout_lock).unwrap();
}